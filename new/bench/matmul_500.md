| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 500` | 18.2 ± 2.1 | 13.7 | 22.9 | 1.00 |
| `./bin_cpp/matmul 500` | 24.5 ± 1.8 | 21.2 | 27.6 | 1.35 ± 0.18 |
| `python src/py/matmul.py 500` | 99.5 ± 5.9 | 87.4 | 114.0 | 5.46 ± 0.69 |
