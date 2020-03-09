#include <iostream>
#include <sstream>
#include <string>
#include <armadillo>

using namespace std;
using namespace arma;

int main(int argc, char** argv) {
    string str(argv[1]);
    unsigned long n;
    stringstream(str) >> n;

    arma_rng::set_seed_random();

    mat a = randu<mat>(n, n);
    cout << det(a) << endl;

    return 0;
}

