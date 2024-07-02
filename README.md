_Scroll to bottom for comparison table_

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

----------------------------------------------------------------------------------------------------------------------------------------------------------------------------
| configuration        | readers | writers |  reads/sec | writes/sec | read p95 (ms) | read p99 (ms) | read p99.9 (ms) | write p95 (ms) | write p99 (ms) | write p99.9 (ms) |
|----------------------|---------|---------|------------|------------|---------------|---------------|-----------------|----------------|----------------|------------------|
| baseline             |       1 |       0 | 19,721.40  | N/A        |        0.0519 |        0.0703 |           0.369 | N/A            | N/A            | N/A              |
| baseline             |       1 |       1 | 51.00      | 297.40     |       84.35   |       183.65  |          541.05 | 4.95           | **5.70**       | **8.61**         |
| baseline             |       1 |       2 | 52.20      | 284.00     |       81.66   |       432.48  |          540.23 | 5.17           | **6.01**       | **13.84**        |
| baseline             |       1 |       3 | 78.00      | 275.40     |       38.94   |       135.66  |          338.04 | 5.44           | **6.33**       | 5,058.23         |
|                      |         |         |            |            |               |               |                 |                |                |                  |
| wal                  |       1 |       0 | **120,902.80** | N/A    |    **0.0079** |    **0.009**  |       **0.018** | N/A            | N/A            | N/A              |
| wal                  |       1 |       1 | **2,375.60** | **624.80** |  **0.95**   |    **1.18**   |        **1.67** | **2.06**       | 8.59           | 17.99            |
| wal                  |       1 |       2 | **1,313.60** | **544.80** |  **1.21**   |    **1.49**   |        **2.09** | **4.07**       | 23.12          | 442.02           |
| wal                  |       1 |       3 | **1,212.00** | **533.40** |  **1.26**   |    **1.58**   |        **2.21** | **5.51**       | 86.20          | 940.48           |
|                      |         |         |            |            |               |               |                 |                |                |                  |
| shared cache         |       1 |       0 | 19,131.20  | N/A        |        0.0567 |        0.0697 |           0.384 | N/A            | N/A            | N/A              |
| shared cache         |       1 |       1 | 425.20     | 264.00     |        4.54   |        8.27   |           14.10 | 4.98           | 5.86           | 11.52            |
| shared cache         |       1 |       2 | 412.60     | 238.60     |        8.09   |       14.85   |           21.94 | 15.49          | 21.48          | 27.48            |
| shared cache         |       1 |       3 | 257.20     | 249.80     |       12.57   |       29.99   |           45.89 | 24.02          | 34.48          | 48.12            |
|                      |         |         |            |            |               |               |                 |                |                |                  |
| wal + shared cache   |       1 |       0 | 117,378.00 | N/A        |        0.0082 |        0.0095 |           0.018 | N/A            | N/A            | N/A              |
| wal + shared cache   |       1 |       1 | 635.00     | 570.20     |        2.68   |       18.92   |           24.35 | 2.37           | 19.12          | 23.95            |
| wal + shared cache   |       1 |       2 | 735.20     | 424.80     |        3.41   |        6.03   |           27.30 | 8.54           | 26.77          | 30.87            |
| wal + shared cache   |       1 |       3 | 795.20     | 395.40     |        4.71   |        8.58   |           30.67 | 19.19          | 32.71          | **42.57**        |
