peroxide="./Rust/target/release/"
dnumeric="./D/d"
R="Rscript R/"
Julia="julia Julia/"
numpy="python Python/"

bench () {
  hyperfine -w 3 --export-csv data/$1.csv --export-markdown data/$1.md "${peroxide}$1" "${dnumeric}" "${R}$1.R" "${numpy}$1.py" "${Julia}$1.jl"
}

bench $1
