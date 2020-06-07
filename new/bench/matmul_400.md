| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 400` | 12.3 ± 1.1 | 8.9 | 14.1 | 1.00 |
| `./bin_cpp/matmul 400` | 13.6 ± 1.0 | 11.7 | 15.9 | 1.10 ± 0.13 |
| `python src/py/matmul.py 400` | 97.1 ± 4.2 | 88.5 | 104.7 | 7.90 ± 0.78 |
