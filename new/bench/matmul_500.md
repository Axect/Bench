| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 500` | 18.9 ± 0.5 | 17.9 | 20.6 | 1.00 |
| `./bin_cpp/matmul 500` | 19.6 ± 2.3 | 17.9 | 43.7 | 1.04 ± 0.13 |
| `python src/py/matmul.py 500` | 96.7 ± 5.1 | 93.5 | 118.7 | 5.12 ± 0.30 |
