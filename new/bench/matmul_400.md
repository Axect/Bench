| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 400` | 13.1 ± 3.1 | 9.9 | 54.9 | 1.00 |
| `./bin_cpp/matmul 400` | 14.0 ± 0.9 | 12.6 | 15.6 | 1.06 ± 0.26 |
| `python src/py/matmul.py 400` | 94.5 ± 2.0 | 90.8 | 99.0 | 7.20 ± 1.69 |
