#include <vector>
#include <algorithm>
#include <iostream>

using namespace std;

struct Generator {
    Generator(int n) : val(n) {}
    int operator()() {
        int old = val;
        val += 2;
        return old;
    }

    int val;
};

int main() {
    const int n = 1000000;
    // v[i] = 2*i + 1
    vector<int> v(n);
    generate(v.begin(), v.end(), Generator(1));
   
    long double mean = 0;

    for (auto it=v.begin(); it!=v.end(); ++it) {
        mean += *it;
    }

    cout << mean/n << endl;
}
