| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 1000` | 72.9 ± 2.9 | 69.5 | 90.2 | 1.14 ± 0.11 |
| `./bin_cpp/matmul 1000` | 64.1 ± 5.5 | 56.5 | 81.9 | 1.00 |
| `python src/py/matmul.py 1000` | 124.8 ± 5.1 | 116.5 | 131.0 | 1.95 ± 0.19 |
