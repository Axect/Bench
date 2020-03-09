extern crate peroxide;
use peroxide::*;

fn main() {
    let a = rand(100, 100);
    let b = rand(100, 1);

    let x = solve(&a, &b);
    x.unwrap().print();
}