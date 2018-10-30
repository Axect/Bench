import std.stdio;
import dnum.tensor;
import dnum.linalg;
import dnum.utils;

void main() {
  auto a = Tensor([[1, 2, 2], [4, 5, 1], [7, 2, 9]]);
  a.inv.writeln;
}
