| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/matmul 200` | 7.0 ± 2.4 | 3.4 | 10.7 | 2.37 ± 0.85 |
| `./bin_cpp/matmul 200` | 2.9 ± 0.3 | 2.5 | 4.2 | 1.00 |
| `python src/py/matmul.py 200` | 89.8 ± 4.4 | 80.7 | 99.9 | 30.48 ± 3.67 |
