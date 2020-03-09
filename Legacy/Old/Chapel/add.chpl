proc main() {
    var a: [0..#100, 0..#100] real;
    for i in 0..#100 {
        for j in 0..#100 {
            a[i, j] = i*100 + j + 1;
        }
    }
    writeln(a + a);
}