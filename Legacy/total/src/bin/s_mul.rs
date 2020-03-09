extern crate peroxide;
use peroxide::*;

fn main() {
    let a = seq(0, 10, 1e-5);
    a.fmap(|x| x * 2f64).print();
}
