extern crate peroxide;
use peroxide::*;

pub fn main() {
    let a = seq(1, 10_000_000, 1);
    a.into_iter()
        .map(|x| 2. * x + 1.)
        .fold(0., |x, y| x + y).print();
}
