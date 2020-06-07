| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 10` | 1.7 ± 0.1 | 1.5 | 2.2 | 2.02 ± 0.35 |
| `./bin_cpp/matmul 10` | 0.8 ± 0.1 | 0.7 | 1.7 | 1.00 |
| `python src/py/matmul.py 10` | 77.8 ± 1.7 | 75.9 | 83.4 | 91.59 ± 13.99 |
