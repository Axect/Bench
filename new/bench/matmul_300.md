| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 300` | 8.1 ± 2.6 | 6.3 | 49.9 | 1.17 ± 0.39 |
| `./bin_cpp/matmul 300` | 6.9 ± 0.5 | 6.1 | 8.2 | 1.00 |
| `python src/py/matmul.py 300` | 92.1 ± 3.8 | 87.9 | 110.2 | 13.34 ± 1.07 |
