extern crate peroxide;
use peroxide::*;

fn main() {
    let a = zeros(1000, 1000);
    (&a + &a).print();
}