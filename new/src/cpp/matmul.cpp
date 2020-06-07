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
    MatrixXd b = MatrixXd::Random(n, n);

    cout << (a * b)(n-1, n-1) << endl;

    return 0;
}

