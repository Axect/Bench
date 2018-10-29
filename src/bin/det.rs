extern crate peroxide;
use peroxide::*;

pub fn main() {
    let a = matrix!(1;200;2, 10, 10, Row);
    println!("{}", a.det());
}
