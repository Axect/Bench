#include <iostream>
#include <sstream>
#include <string>
#include <Eigen/Dense>

using namespace std;
using namespace Eigen;

int main(int argc, char** argv) {
    string str(argv[1]);
    unsigned long n;
    stringstream(str) >> n;

    srand((unsigned int) time(0));
    MatrixXd a = MatrixXd::Random(n, n);
    cout << a.determinant() << endl;

    return 0;
}

