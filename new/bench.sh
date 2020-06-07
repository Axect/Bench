# Build
RustFLAGS="-C target-cpu=native" cargo build --release
g++ -O2 -o bin_cpp/det src/cpp/det.cpp -I/usr/include/eigen3
g++ -O2 -o bin_cpp/matmul src/cpp/matmul.cpp -I/usr/include/eigen3

# Benchmark
ORDER="10 100 200 300 400 500"
for i in $ORDER
do
    hyperfine -w 3 --export-markdown "bench/det_$i.md" "./target/release/det $i" "./target/release/det_nd $i" "./bin_cpp/det $i" "python src/py/det.py $i"
    hyperfine -w 3 --export-markdown "bench/matmul_$i.md" "./target/release/matmul $i" "./bin_cpp/matmul $i" "python src/py/matmul.py $i"
done
