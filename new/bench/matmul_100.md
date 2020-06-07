| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 100` | 6.3 ± 2.8 | 2.1 | 10.4 | 4.33 ± 2.01 |
| `./bin_cpp/matmul 100` | 1.5 ± 0.2 | 1.1 | 2.8 | 1.00 |
| `python src/py/matmul.py 100` | 90.8 ± 4.9 | 79.5 | 98.6 | 62.47 ± 9.95 |
