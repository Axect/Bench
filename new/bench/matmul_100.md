| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 100` | 2.5 ± 0.8 | 2.1 | 10.0 | 1.00 |
| `./bin_cpp/matmul 100` | 7.1 ± 2.5 | 6.0 | 30.3 | 2.87 ± 1.40 |
| `python src/py/matmul.py 100` | 86.4 ± 1.6 | 80.9 | 91.6 | 34.86 ± 11.74 |
