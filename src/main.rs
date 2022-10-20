use random_fast_rng::{FastRng, Random};
use rusqlite::{params, Connection, DropBehavior};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

const USE_WAL: bool = true;
const USE_RWLOCK: bool = false;
const SEED_COUNT: usize = 20;
const NEW_ITEM_SIZE: usize = 40 * 1024;
const PRINT_VALUES: bool = false;
/// SQLite's approach to concurrency requires waiting/backing off in case of
/// readers/writers conflict. This sets a max duration before failing.
const DB_TIMEOUT: Duration = Duration::from_secs(6);

struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn create<P: AsRef<Path>>(path: P) -> Self {
        let path: &Path = path.as_ref();
        if path.exists() {
            fs::remove_file(path).expect("Could not delete existing database file");
        }

        let mut db = Self::open(path);
        db.create_tables();

        db
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Self {
        let conn = Connection::open(path).expect("Could not create SQLite connection");
        conn.busy_timeout(DB_TIMEOUT)
            .expect("Error setting the database timeout");

        Database { conn }
    }

    fn create_tables(&mut self) {
        if USE_WAL {
            self.conn
                .pragma_update(None, "journal_mode", &"WAL".to_owned())
                .expect("Error applying WAL journal_mode");
        }
        self.conn
            .execute(
                r#"
CREATE TABLE "kv" (
	"key"	INTEGER NOT NULL,
	"value"	BLOB NOT NULL,
	PRIMARY KEY("key")
) WITHOUT ROWID;
"#,
                params![],
            )
            .expect("Error creating tables");
    }

    pub fn seed(&mut self) -> std::io::Result<Vec<u16>> {
        let mut transaction = self
            .conn
            .transaction()
            .expect("Could not open DB transaction");
        transaction.set_drop_behavior(DropBehavior::Commit);

        let mut query = transaction
            .prepare(
                r#"
INSERT INTO "kv" VALUES (?1, ?2);
"#,
            )
            .expect("Failed to prepare insert query");

        let mut keys = Vec::new();
        let mut rng = FastRng::new();
        for k in &mut keys {
            *k = rng.get_u16();
        }

        for _ in 0..SEED_COUNT {
            let (key, value) = (rng.get_u16(), rng.get_u16());
            keys.push(key);
            query
                .execute(params![key, value])
                .expect("Insertion failure seeding database!");
        }

        Ok(keys)
    }
}

fn read_loop(
    db: Database,
    keys: &[u16],
    stop: Arc<AtomicBool>,
    rwlock: Arc<RwLock<()>>,
) -> Vec<i64> {
    let mut times = Vec::new();

    let mut query = db
        .conn
        .prepare(
            r#"
SELECT "value" FROM "kv"
WHERE "key" = ?1
LIMIT 1;"#,
        )
        .expect("Failed to prepare query statement");

    let mut rng = FastRng::new();
    while !stop.load(Ordering::Relaxed) {
        let key_index = rng.get_usize() % keys.len();
        let key = &keys[key_index as usize];

        let timer = Instant::now();
        let _guard;
        if USE_RWLOCK {
            _guard = rwlock.read().expect("Cannot unlock for read!");
        }
        let value: Result<String, _> = query.query_row(&[key], |result| result.get(0));
        let elapsed = timer.elapsed();
        if PRINT_VALUES {
            if let Ok(value) = value {
                println!("{}: {}", key, value);
            }
        }
        times.push(elapsed.as_nanos() as i64);
    }

    times
}

fn write_loop(db: Database, stop: Arc<AtomicBool>, rwlock: Arc<RwLock<()>>) -> Vec<i64> {
    let mut times = Vec::new();

    let mut query = db
        .conn
        .prepare(
            r#"
INSERT OR IGNORE INTO "kv" ("key", "value")
VALUES (?1, ?2)
"#,
        )
        .expect("Failed to prepare update statement");

    let mut rng = FastRng::new();
    let mut value = Vec::new();
    value.resize(NEW_ITEM_SIZE, 0u8);
    rng.fill_bytes(&mut value);

    while !stop.load(Ordering::Relaxed) {
        let key = rng.get_u16();

        let timer = Instant::now();
        let _guard;
        if USE_RWLOCK {
            _guard = rwlock.write().expect("Cannot unlock for read!");
        }
        let rows_updated = query
            .execute(params![key, value])
            .expect("Failed to issue update query!");
        let elapsed = timer.elapsed();
        if PRINT_VALUES && rows_updated > 0 {
            println!("{} set", key);
        }
        times.push(elapsed.as_nanos() as i64);
    }

    times
}

fn average(nums: &[i64]) -> f64 {
    let sum: i128 = nums.iter().map(|n| *n as i128).sum();
    sum as f64 / (nums.len() as f64)
}

fn main() {
    let keys = {
        let mut db = Database::create("test.db");
        db.seed().expect("Error seeding database!")
    };

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
                sender
                    .send(write_times.len())
                    .expect("Could not send write count!");
            });
        }
        drop(write_counts_send);

        let mut read_times = read_loop(db, &keys, done.clone(), rwlock.clone());
        read_times.sort();

        let mut total_writes = 0;
        for _ in 0..writers {
            let writes = write_counts_recv
                .recv()
                .expect("Failed to receive write counts!");
            total_writes += writes;
        }

        eprintln!("{} writers:", writers);
        eprintln!("- Read {} values from the database.", read_times.len());
        eprintln!("- Wrote {} values to the database.", total_writes);
        eprintln!(
            "- Mean read time: {:.5} ms",
            average(&read_times) / 1000_000f64
        );
        let p95_nanos = read_times[(0.95 * (read_times.len() as f64)) as usize];
        eprintln!("- P95: {} ms", p95_nanos as f64 / 1000_000f64);
        let p99_nanos = read_times[(0.99 * (read_times.len() as f64)) as usize];
        eprintln!("- P99: {} ms", p99_nanos as f64 / 1000_000f64);
        let p99_9_nanos = read_times[(0.999 * (read_times.len() as f64)) as usize];
        eprintln!("- P99.9: {} ms", p99_9_nanos as f64 / 1000_000f64);
        eprintln!("");
    }
}
