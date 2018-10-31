import std.stdio;
import dnum.tensor;
import dnum.linalg;
import dnum.utils;

void main() {
  // auto a = Tensor([[1, 2, 2], [4, 5, 1], [7, 2, 9]]);
  // a.det.writeln;
  // a.inv.writeln;
  auto a = Tensor(100, 100);
  foreach (i; 0 .. 100) {
    foreach (j; 0 .. 100) {
      a[i, j] = i * 100 + j + 1;
    }
  }
  (a % a).writeln;
}
