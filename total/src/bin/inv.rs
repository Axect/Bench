extern crate peroxide;
use peroxide::*;

fn main() {
    let a = rand(100, 100);
    a.inv().unwrap().print();
}