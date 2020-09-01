| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 200` | 4.7 ± 1.9 | 3.1 | 19.6 | 1.57 ± 0.65 |
| `./bin_cpp/matmul 200` | 3.0 ± 0.3 | 2.5 | 3.9 | 1.00 |
| `python src/py/matmul.py 200` | 90.1 ± 2.5 | 86.6 | 95.9 | 30.26 ± 2.84 |
