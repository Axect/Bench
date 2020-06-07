| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 300` | 8.6 ± 2.2 | 5.6 | 30.9 | 1.32 ± 0.35 |
| `./bin_cpp/matmul 300` | 6.5 ± 0.5 | 5.7 | 8.2 | 1.00 |
| `python src/py/matmul.py 300` | 93.0 ± 5.4 | 82.4 | 108.7 | 14.28 ± 1.34 |
