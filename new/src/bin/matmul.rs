use peroxide::fuga::*;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let n: usize = args[1].parse().unwrap();
    let a = rand(n, n);
    let b = rand(n, n);

    (&a * &b)[(n-1, n-1)].print();
}
