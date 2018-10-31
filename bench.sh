peroxide="./target/release/"
dnumeric="./D/d"
R="Rscript R/"
Julia="julia Julia/"
numpy="python Python/"

bench () {
  hyperfine -w 3 --export-csv data/${bench_target}.csv --export-markdown data/${bench_target}.md "${peroxide}${bench_target}" "${dnumeric}" "${R}${bench_target}.R" "${numpy}${bench_target}.py" "${Julia}${bench_target}.jl"
}

# Det

export bench_target="det"

bench
