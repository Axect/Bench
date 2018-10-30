| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./target/release/inv` | 0.6 ± 1.1 | 0.0…5.0 |
| `Rscript R/inv.R` | 137.9 ± 8.5 | 123.0…161.6 |
| `julia --compiled-modules=yes -O3 Julia/inv.jl` | 662.9 ± 14.6 | 638.8…692.9 |
| `./D/d` | 3.7 ± 1.7 | 2.2…11.5 |
