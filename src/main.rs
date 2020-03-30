use std::path::Path;
use random_fast_rng::{FastRng, Random};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, RwLock};
use rusqlite::{Connection, DropBehavior, params};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::{self, File};

const WORD_FILE: &'static str = "/usr/share/dict/words";

struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let path: &Path = path.as_ref();
        if path.exists() {
            fs::remove_file(path)
                .expect("Could not delete existing database file");
        }

        let mut db = Self::open(path);
        db.create_tables();

        db
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        Database {
            conn: Connection::open(path)
                .expect("Could not create SQLite connection"),
        }
    }

    fn create_tables(&mut self) {
        self.conn.pragma_update(None, "journal_mode", &"WAL".to_owned())
            .expect("Error applying WAL journal_mode");
        self.conn.execute(r#"
CREATE TABLE "kv" (
	"key"	TEXT NOT NULL,
	"value"	TEXT,
	PRIMARY KEY("key")
) WITHOUT ROWID;
"#, params![])
            .expect("Error creating tables");
    }

    pub fn seed(&mut self) -> std::io::Result<()> {
        let mut transaction = self.conn.transaction()
            .expect("Could not open DB transaction");
        transaction.set_drop_behavior(DropBehavior::Commit);

        let mut query = transaction.prepare(r#"
INSERT INTO "kv" VALUES (?1, ?2);
"#).expect("Failed to prepare insert query");

        let file = File::open(WORD_FILE)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        loop {
            let (key, value) = match (lines.next(), lines.next()) {
                (Some(Ok(key)), Some(Ok(value))) => (key, value),
                _ => break,
            };

            query.execute(&[key, value])
                .expect("Insertion failure seeding database!");
        }

        Ok(())
    }
}

fn read_loop(db: Database, stop: Arc<AtomicBool>, _rwlock: Arc<RwLock<()>>) -> Vec<i64> {
    let mut times = Vec::new();

    let file = File::open(WORD_FILE)
        .expect("Failed to open words file");
    let reader = BufReader::new(file);
    let words: Vec<_> = reader.lines()
        .map(|line| line.expect("Error reading from word file!"))
        .collect();

    let mut query = db.conn.prepare(r#"
SELECT "value" FROM "kv"
WHERE "key" = ?1
LIMIT 1;"#)
        .expect("Failed to prepare query statement");

    let mut rng = FastRng::new();
    while !stop.load(Ordering::Acquire) {
        let key_index: i32 = rng.gen();
        if key_index as usize >= words.len() {
            continue;
        }
        let key = &words[key_index as usize];

        let timer = Instant::now();
        let value: Result<String, _> = query.query_row(&[key], |result| result.get(0));
        let elapsed = timer.elapsed();
        if let Ok(value) = value {
            println!("{}: {}", key, value);
        }
        times.push(elapsed.as_nanos() as i64);
    }


    times
}

fn write_loop(db: Database, stop: Arc<AtomicBool>, _rwlock: Arc<RwLock<()>>) -> Vec<i64> {
    let mut times = Vec::new();

    let file = File::open(WORD_FILE)
        .expect("Failed to open words file");
    let reader = BufReader::new(file);
    let words: Vec<_> = reader.lines()
        .map(|line| line.expect("Error reading from word file!"))
        .collect();

    let mut query = db.conn.prepare(r#"
UPDATE "kv"
SET "value" = ?2
WHERE "key" = ?1
LIMIT 1;"#)
        .expect("Failed to prepare update statement");

    let mut rng = FastRng::new();
    while !stop.load(Ordering::Acquire) {
        let mut key_index = (words.len() + 1) as i32;
        while key_index as usize >= words.len() {
            key_index = rng.gen();
        }
        let mut value_index = (words.len() + 1) as i32;
        while value_index as usize >= words.len() {
            value_index = rng.gen();
        }
        let key = &words[key_index as usize];
        let value = &words[value_index as usize];

        let timer = Instant::now();
        let rows_updated = query.execute(&[key, value])
            .expect("Failed to issue update query!");
        let elapsed = timer.elapsed();
        if rows_updated > 0 {
            println!("{} set to {}", key, value);
        } else {
            println!("{} not found", key);
        }
        times.push(elapsed.as_nanos() as i64);
    }

    times
}

fn average(nums: &[i64]) -> i64 {
    let sum: i128 = nums.iter().map(|n| *n as i128).sum();
    (sum / (nums.len() as i128)) as i64
}

fn main() {
    {
        let mut db = Database::create("test.db");
        db.seed().expect("Error seeding database!");
    }

    for writers in 0..4 {
        let done = Arc::new(AtomicBool::new(false));
        let rwlock = Arc::new(RwLock::new(()));

        {
            let done = done.clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(5));
                done.store(true, Ordering::Release);
            });
        }

        let db = Database::open("test.db");
        let (write_counts_send, write_counts_recv) = mpsc::channel();
        for _ in 0..writers {
            let done = done.clone();
            let sender = write_counts_send.clone();
            let rwlock = rwlock.clone();
            thread::spawn(move || {
                let write_db = Database::open("test.db");
                let write_times = write_loop(write_db, done, rwlock);
                sender.send(write_times.len())
                    .expect("Could not send write count!");
            });
        }
        drop(write_counts_send);

        let read_times = read_loop(db, done.clone(), rwlock.clone());
        let mut sorted_times = read_times.clone();
        sorted_times.sort();

        let mut total_writes = 0;
        for _ in 0..writers {
            let writes = write_counts_recv.recv().expect("Failed to receive write counts!");
            total_writes += writes;
        }

        eprintln!("{} writers:", writers);
        eprintln!("- Read {} values from the database.", read_times.len());
        eprintln!("- Wrote {} values to the database.", total_writes);
        eprintln!("- Mean response time: {} ns", average(&read_times));
        let p95_nanos = sorted_times[(0.95 * (sorted_times.len() as f64)) as usize];
        eprintln!("- P95: {} ns", p95_nanos);
        let p99_nanos = sorted_times[(0.99 * (sorted_times.len() as f64)) as usize];
        eprintln!("- P99: {} ns", p99_nanos);
        eprintln!("");
    }
}
