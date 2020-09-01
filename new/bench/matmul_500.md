| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 500` | 20.9 ± 4.4 | 14.9 | 48.7 | 1.00 |
| `./bin_cpp/matmul 500` | 26.2 ± 1.5 | 22.7 | 30.8 | 1.26 ± 0.27 |
| `python src/py/matmul.py 500` | 102.6 ± 6.4 | 95.9 | 131.3 | 4.92 ± 1.08 |
