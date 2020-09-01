| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 10` | 2.3 ± 0.1 | 1.5 | 2.7 | 2.68 ± 0.36 |
| `./bin_cpp/matmul 10` | 0.8 ± 0.1 | 0.7 | 1.6 | 1.00 |
| `python src/py/matmul.py 10` | 84.2 ± 1.4 | 79.8 | 86.9 | 99.60 ± 12.66 |
