read BENCH

export RUST_BENCH="cargo run --release --bin $BENCH"
export JULIA_BENCH="julia julia/$BENCH.jl"
export PYTHON_BENCH="python python/$BENCH.py"

hyperfine -w 5 --export-markdown result/$BENCH.md --export-csv result/$BENCH.csv "$RUST_BENCH" "$JULIA_BENCH" "$PYTHON_BENCH"