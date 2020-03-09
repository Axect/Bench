extern crate peroxide;
use peroxide::*;

fn main() {
    // Row matrix
    let a = matrix!(1;10000;1, 100, 100, Row);
    //println!("{}", a.clone() % a);
    // Col matrix
    let b = matrix!(1;10000;1, 100, 100, Col);
    println!("{}", a % b);
}
