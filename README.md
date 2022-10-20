## DbOptions { wal: false, shared_cache: false }

0 writers:
- Read 98607 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.05011 ms
- P95: 0.0519 ms
- P99: 0.0703 ms
- P99.9: 0.3693 ms

1 writers:
- Read 255 values from the database.
- Wrote 1487 values to the database.
- Mean read time: 19.62688 ms
- P95: 84.3469 ms
- P99: 183.647 ms
- P99.9: 541.0497 ms

2 writers:
- Read 261 values from the database.
- Wrote 1420 values to the database.
- Mean read time: 19.23322 ms
- P95: 81.6596 ms
- P99: 432.4794 ms
- P99.9: 540.2341 ms

3 writers:
- Read 390 values from the database.
- Wrote 1377 values to the database.
- Mean read time: 12.83637 ms
- P95: 38.9406 ms
- P99: 135.6645 ms
- P99.9: 338.0449 ms

## DbOptions { wal: true, shared_cache: false }

0 writers:
- Read 604514 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.00783 ms
- P95: 0.0079 ms
- P99: 0.009 ms
- P99.9: 0.0179 ms

1 writers:
- Read 11878 values from the database.
- Wrote 3124 values to the database.
- Mean read time: 0.42012 ms
- P95: 0.9454 ms
- P99: 1.1835 ms
- P99.9: 1.6715 ms

2 writers:
- Read 6568 values from the database.
- Wrote 2724 values to the database.
- Mean read time: 0.76049 ms
- P95: 1.211 ms
- P99: 1.4856 ms
- P99.9: 2.0935 ms

3 writers:
- Read 6060 values from the database.
- Wrote 2667 values to the database.
- Mean read time: 0.82431 ms
- P95: 1.2649 ms
- P99: 1.5833 ms
- P99.9: 2.2149 ms

## DbOptions { wal: false, shared_cache: true }

0 writers:
- Read 95656 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.05172 ms
- P95: 0.0567 ms
- P99: 0.0697 ms
- P99.9: 0.3842 ms

1 writers:
- Read 2126 values from the database.
- Wrote 1320 values to the database.
- Mean read time: 2.35140 ms
- P95: 4.5449 ms
- P99: 8.2737 ms
- P99.9: 14.1016 ms

2 writers:
- Read 2063 values from the database.
- Wrote 1193 values to the database.
- Mean read time: 2.42350 ms
- P95: 8.0921 ms
- P99: 14.8512 ms
- P99.9: 21.9369 ms

3 writers:
- Read 1286 values from the database.
- Wrote 1249 values to the database.
- Mean read time: 3.89373 ms
- P95: 12.5728 ms
- P99: 29.9939 ms
- P99.9: 45.888 ms

## DbOptions { wal: true, shared_cache: true }

0 writers:
- Read 586890 values from the database.
- Wrote 0 values to the database.
- Mean read time: 0.00809 ms
- P95: 0.0082 ms
- P99: 0.0095 ms
- P99.9: 0.018 ms

1 writers:
- Read 3175 values from the database.
- Wrote 2851 values to the database.
- Mean read time: 1.57313 ms
- P95: 2.682 ms
- P99: 18.9191 ms
- P99.9: 24.346 ms

2 writers:
- Read 3676 values from the database.
- Wrote 2124 values to the database.
- Mean read time: 1.35989 ms
- P95: 3.4124 ms
- P99: 6.031 ms
- P99.9: 27.2993 ms

3 writers:
- Read 3976 values from the database.
- Wrote 1977 values to the database.
- Mean read time: 1.25674 ms
- P95: 4.709 ms
- P99: 8.5832 ms
- P99.9: 30.6743 ms

---------------------------------

| configuration | readers | writers | reads/sec | writes/sec | read p95 (ms) | read p99 | read p99.9 | write p95 | write p99 | write p99.9 |
| ------------- | ------- | ------- | --------- | ---------- | ------------- | -------- | ---------- | --------- | --------- | ----------- |
| !wal, !shared_cache |  1 |  0 | 19721.4 | 0 | 0.0519 | 0.0703 | 0.3693 | N/A | N/A | N/A |
| !wal, !shared_cache |  1 |  1 | 51 | 297.4 | 84.3469 | 183.647 | 541.0497 | 4.946 | 5.6973 | 8.6103 |
| !wal, !shared_cache |  1 |  2 | 52.2 | 284 | 81.6596 | 432.4794 | 540.2341 | 5.1718 | 6.0096 | 13.8398 |
| !wal, !shared_cache |  1 |  3 | 78 | 275.4 | 38.9406 | 135.6645 | 338.0449 | 5.4437 | 6.3292 | 5058.2298 |
| wal, !shared_cache  |  1 |  0 | 120902.8 | 0 | 0.0079 | 0.009 | 0.0179 | N/A | N/A | N/A |
| wal, !shared_cache  |  1 |  1 | 2375.6 | 624.8 | 0.9454 | 1.1835 | 1.6715 | 2.0587 | 8.5901 | 17.9911 |
| wal, !shared_cache  |  1 |  2 | 1313.6 | 544.8 | 1.211 | 1.4856 | 2.0935 | 4.0653 | 23.1239 | 442.0249 |
| wal, !shared_cache  |  1 |  3 | 1212 | 533.4 | 1.2649 | 1.5833 | 2.2149 | 5.5132 | 86.2031 | 940.4805 |
| !wal, shared_cache  |  1 |  0 | 19131.2 | 0 | 0.0567 | 0.0697 | 0.3842 | N/A | N/A | N/A |
| !wal, shared_cache  |  1 |  1 | 425.2 | 264 | 4.5449 | 8.2737 | 14.1016 | 4.9789 | 5.8592 | 11.5163 |
| !wal, shared_cache  |  1 |  2 | 412.6 | 238.6 | 8.0921 | 14.8512 | 21.9369 | 15.4856 | 21.4848 | 27.484 |
| !wal, shared_cache  |  1 |  3 | 257.2 | 249.8 | 12.5728 | 29.9939 | 45.888 | 24.0153 | 34.4784 | 48.1155 |
| wal, shared_cache   |  1 |  0 | 117378 | 0 | 0.0082 | 0.0095 | 0.018 | N/A | N/A | N/A |
| wal, shared_cache   |  1 |  1 | 635 | 570.2 | 2.682 | 18.9191 | 24.346 | 2.3667 | 19.1163 | 23.9537 |
| wal, shared_cache   |  1 |  2 | 735.2 | 424.8 | 3.4124 | 6.031 | 27.2993 | 8.5401 | 26.7671 | 30.8671 |
| wal, shared_cache   |  1 |  3 | 795.2 | 395.4 | 4.709 | 8.5832 | 30.6743 | 19.1934 | 32.7071 | 42.5692 |
