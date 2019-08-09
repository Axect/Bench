#[macro_use]
extern crate criterion;
extern crate peroxide;
use peroxide::*;

use criterion::Criterion;

fn gram() {
    // Make random matrix
    let mut s = rand(100, 100);
    for i in 0 .. 100 {
        s[(i,i)] = s[(i,i)] * 10f64;
        for j in 0 .. i {
            s[(i,j)] = s[(j,i)];
        }
    }

    let mut ds = gram_schmidt(&s);

    for _i in 0 .. 10 {
        s = ds.t() * s.clone() * ds;
        ds = gram_schmidt(&s);
    }
    s.print();
}

fn bench_gram(c: &mut Criterion) {
    c.bench_function("gram_schmidt", |b| b.iter(|| gram()));
}

fn proj(u: &Vec<f64>, v: &Vec<f64>) -> Vec<f64> {
    let uv = u.dot(v);
    let uu = u.dot(u);
    u.fmap(|x| x * uv / uu)
}

fn gram_schmidt(vs: &Matrix) -> Matrix {
    let mut result: Matrix = zeros_shape(vs.row, vs.col, Col);
    result.subs_col(0, vs.col(0));

    for k in 1 .. vs.col {
        let vk = &vs.col(k);
        let mut puv = vec![0f64; vk.len()];
        for j in 0 .. k {
            puv = puv.add(&proj(&result.col(j), vk));
        }
        result.subs_col(k, vk.sub(&puv));
    }

    result.col_map(|v| v.normalize())
}

criterion_group!(benches, bench_gram);
criterion_main!(benches);