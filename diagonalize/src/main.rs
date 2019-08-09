extern crate peroxide;
use peroxide::*;

fn main() {
    let v1 = c!(3,1);
    let v2 = c!(2,2);

    let pv2 = proj(&v1, &v2);
    pv2.print();

    let s = vec![v1, v2];

    let ds = gram_schmidt(&s);

    ds.into_iter().for_each(|x| x.print());
}

fn proj(u: &Vec<f64>, v: &Vec<f64>) -> Vec<f64> {
    let uv = u.dot(v);
    let uu = u.dot(u);
    u.fmap(|x| x * uv / uu)
}

fn gram_schmidt(vs: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = vec![vec![0f64; vs[0].len()]; vs.len()];
    result[0] = vs[0].clone();

    for k in 1 .. vs.len() {
        let vk = &vs[k];
        let mut puv = vec![0f64; vk.len()];
        for j in 0 .. k {
            puv = puv.add(&proj(&result[j], vk));
        }
        result[k] = vk.sub(&puv);
    }

    result.into_iter().map(|v| {
        let n = v.norm();
        v.fmap(|t| t / n)
    }).collect()
}