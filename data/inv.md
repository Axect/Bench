| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./target/release/inv` | 0.6 ± 1.1 | 0.0…5.8 |
| `Rscript R/inv.R` | 138.6 ± 7.4 | 126.4…160.2 |
| `julia --compiled-modules=yes -O3 Julia/inv.jl` | 683.4 ± 18.0 | 644.1…703.3 |
| `./D/d` | 3.8 ± 1.8 | 2.3…13.2 |
| `python Python/inv.py` | 107.0 ± 10.3 | 95.1…136.7 |
