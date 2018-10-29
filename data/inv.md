| Command | Mean [ms] | Min…Max [ms] |
|:---|---:|---:|
| `./target/release/inv` | 0.5 ± 0.3 | 0.2…2.5 |
| `Rscript R/inv.R` | 116.8 ± 4.0 | 112.2…130.1 |
| `julia --compiled-modules=yes -O3 Julia/inv.jl` | 557.9 ± 7.5 | 546.4…568.4 |
