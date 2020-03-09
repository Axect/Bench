import std.stdio;
import dnum.matrix;

void main() {
    // auto a = Tensor([[1, 2, 2], [4, 5, 1], [7, 2, 9]]);
    // a.det.writeln;
    // a.inv.writeln;
    double[] v;
    v.length = 10_000;
    foreach (i; 0 .. 10_000) {
        v[i] = cast(double)(i + 1);
    }
    auto a = Matrix(v, 100, 100, true);
    writeln(a % a);
}
