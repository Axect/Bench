#include <iostream>
#include <sstream>
#include <armadillo>

using namespace std;
using namespace arma;

int main(int argc, char** argv) {
    // Parse input
    unsigned long n;
    string str(argv[1]);
    stringstream(str) >> n;

    // Seed
    arma_rng::set_seed_random();

    // Matmul
    mat a = randu<mat>(n, n);
    mat b = randu<mat>(n, n);

    mat c = a * b;
    
    cout << c(n-1, n-1) << endl;

    return 0;
}
