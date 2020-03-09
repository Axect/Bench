use LinearAlgebra;

proc main() {
    var a = Matrix(100, 100);
    for i in 1 .. #100 {
        for j in 1 .. #100 {
            a[i, j] = (i - 1) * 100 + j;
        }
    }
    writeln(dot(a, a));
}