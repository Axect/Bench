extern crate peroxide;
use peroxide::*;

pub fn main() {
    let a = matrix(c!(1,2,2,4,5,1,7,2,9),3, 3, Row);
    println!("{}", a.det());
}
