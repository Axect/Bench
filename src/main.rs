extern crate peroxide;
use peroxide::*;

fn main() {
    let a = matrix!(1;4;1, 2, 2, Row);
    println!("{}", a);
    println!("{}", a.det());
    println!("{}", a.inv().unwrap());
}
