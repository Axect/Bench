extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_linalg;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use ndarray_linalg::solve::Determinant;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let n: usize = args[1].parse().unwrap();

    let a = Array::random((n, n), Uniform::new(0., 1.));
    println!("{}", a.det().unwrap());
}
