| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 100` | 3.3 ± 1.4 | 2.1 | 10.5 | 2.65 ± 1.19 |
| `./bin_cpp/matmul 100` | 1.2 ± 0.1 | 1.0 | 2.1 | 1.00 |
| `python src/py/matmul.py 100` | 88.5 ± 2.5 | 84.3 | 95.4 | 71.47 ± 7.27 |
