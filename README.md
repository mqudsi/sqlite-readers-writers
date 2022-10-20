## DbOptions { wal: false, shared_cache: false }

0 writers:
- Read 99292 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.04979 ms
- P95: 0.0514 ms
- P99: 0.0618 ms
- P99.9: 0.3715 ms

1 writers:
- Read 298 values from the database.
- Wrote 1456 values to the database.
- Mean read time: 16.78980 ms
- P95: 81.6371 ms
- P99: 238.1666 ms
- P99.9: 339.2873 ms

2 writers:
- Read 308 values from the database.
- Wrote 1387 values to the database.
- Mean read time: 16.38337 ms
- P95: 56.2246 ms
- P99: 134.6607 ms
- P99.9: 1438.1452 ms

3 writers:
- Read 385 values from the database.
- Wrote 1340 values to the database.
- Mean read time: 13.00200 ms
- P95: 37.3729 ms
- P99: 182.2995 ms
- P99.9: 734.2981 ms

## DbOptions { wal: true, shared_cache: false }

0 writers:
- Read 607699 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.00778 ms
- P95: 0.0079 ms
- P99: 0.0089 ms
- P99.9: 0.0191 ms

1 writers:
- Read 12488 values from the database.
- Wrote 2994 values to the database.
- Mean read time: 0.39968 ms
- P95: 1.0081 ms
- P99: 1.3015 ms
- P99.9: 1.7033 ms

2 writers:
- Read 6222 values from the database.
- Wrote 2631 values to the database.
- Mean read time: 0.80288 ms
- P95: 1.2769 ms
- P99: 1.5783 ms
- P99.9: 1.9793 ms

3 writers:
- Read 5870 values from the database.
- Wrote 2656 values to the database.
- Mean read time: 0.85101 ms
- P95: 1.3355 ms
- P99: 1.6838 ms
- P99.9: 2.2956 ms

## DbOptions { wal: false, shared_cache: true }

0 writers:
- Read 97858 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.05056 ms
- P95: 0.0528 ms
- P99: 0.0684 ms
- P99.9: 0.3773 ms

1 writers:
- Read 1692 values from the database.
- Wrote 1335 values to the database.
- Mean read time: 2.95584 ms
- P95: 6.507 ms
- P99: 10.9798 ms
- P99.9: 17.1197 ms

2 writers:
- Read 1391 values from the database.
- Wrote 1252 values to the database.
- Mean read time: 3.59600 ms
- P95: 8.6286 ms
- P99: 15.9738 ms
- P99.9: 24.6031 ms

3 writers:
- Read 1117 values from the database.
- Wrote 1275 values to the database.
- Mean read time: 4.47592 ms
- P95: 12.4364 ms
- P99: 23.6377 ms
- P99.9: 34.2582 ms

## DbOptions { wal: true, shared_cache: true }

0 writers:
- Read 595090 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.00796 ms
- P95: 0.008 ms
- P99: 0.0091 ms
- P99.9: 0.0277 ms

1 writers:
- Read 3441 values from the database.
- Wrote 2787 values to the database.
- Mean read time: 1.45174 ms
- P95: 2.2466 ms
- P99: 15.2858 ms
- P99.9: 23.2502 ms

2 writers:
- Read 3185 values from the database.
- Wrote 2268 values to the database.
- Mean read time: 1.57373 ms
- P95: 3.5658 ms
- P99: 6.4296 ms
- P99.9: 26.7014 ms

3 writers:
- Read 3174 values from the database.
- Wrote 2141 values to the database.
- Mean read time: 1.57426 ms
- P95: 5.01 ms
- P99: 9.4572 ms
- P99.9: 32.3088 ms

------------------------------------------------


| configuration | readers | writers | reads/sec | writes/sec | read p95 (ms) | read p99 | read p99.9 | write p95 | write p99 | write p99.9 |
| ------------- | ------- | ------- | --------- | ---------- | ------------- | -------- | ---------- | --------- | --------- | ----------- |
| !wal, !shared_cache |  1 |  0 |  19858.4 | 0.0 | 0.0514 | 0.0618 | 0.3715 | N/A | N/A |  N/A |
| !wal, !shared_cache |  1 |  1 |     59.6 | 291.2 | 81.6371 | 238.1666 | 339.2873 | 5.09 | 5.93 | 8.76 |
| !wal, !shared_cache |  1 |  2 |     61.6 | 277.4 | 56.2246 | 134.6607 | 1438.1452 | 5.44 | 6.08 | 17.8 |
| !wal, !shared_cache |  1 |  3 |     77.0 | 268.0 | 37.3729 | 182.2995 | 734.2981 | 5.59 | 6.56 | 5059 |
| wal, !shared_cache  |  1 |  0 | 121539.8 | 0.0 | 0.0079 | 0.0089 | 0.0191 | N/A | N/A |  N/A |
| wal, !shared_cache  |  1 |  1 |   2497.6 | 598.8 | 1.0081 | 1.3015 | 1.7033 | 2.25 | 7.02 | 18.0 |
| wal, !shared_cache  |  1 |  2 |   1244.4 | 526.2 | 1.2769 | 1.5783 | 1.9793 | 3.22 | 37.5 | 341. |
| wal, !shared_cache  |  1 |  3 |   1174.0 | 531.2 | 1.3355 | 1.6838 | 2.2956 | 3.15 | 51.8 | 838. |
| !wal, shared_cache  |  1 |  0 |  19571.6 | 0.0 | 0.0528 | 0.0684 | 0.3773 | N/A | N/A |  N/A |
| !wal, shared_cache  |  1 |  1 |    338.4 | 267.0 | 6.5070 | 10.9798 | 17.1197 | 4.89 | 5.94 | 15.1 |
| !wal, shared_cache  |  1 |  2 |    278.2 | 250.4 | 8.6286 | 15.9738 | 24.6031 | 14.4 | 19.0 | 23.5 |
| !wal, shared_cache  |  1 |  3 |    223.4 | 255.0 | 12.4364 | 23.6377 | 34.2582 | 21.3 | 30.1 | 53.8 |
| wal, shared_cache   |  1 |  0 | 119018.0 | 0.0 | 0.0080 | 0.0091 | 0.0277 | N/A | N/A |  N/A |
| wal, shared_cache   |  1 |  1 |    688.2 | 557.4 | 2.2466 | 15.2858 | 23.2502 | 2.54 | 18.5 | 22.6 |
| wal, shared_cache   |  1 |  2 |    637.0 | 453.6 | 3.5658 | 6.4296 | 26.7014 | 8.32 | 26.3 | 32.3 |
| wal, shared_cache   |  1 |  3 |    634.8 | 428.2 | 5.0100 | 9.4572 | 32.3088 | 19.2 | 31.2 | 40.7 |
