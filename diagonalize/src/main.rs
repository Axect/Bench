extern crate peroxide;
extern crate rayon;
use peroxide::*;
use rayon::prelude::*;

pub fn main() {
    // Make random matrix
    let mut s = rand(100, 100);
    for i in 0 .. 100 {
        s[(i,i)] = s[(i,i)] * 10f64;
        for j in 0 .. i {
            s[(i,j)] = s[(j,i)];
        }
    }

    let mut ds = gram_schmidt(&s);

    // matmul(&matmul(&ds.t(),&s), &ds).print();

    for i in 0 .. 500 {
        s = matmul(&matmul(&ds.t(),&s), &ds);
        ds = gram_schmidt(&s);
    }
    s.print();
    
    // let mut s_raw = s.to_vec();
    // let mut ds_raw = gram_schmidt_raw(&s_raw);

    // for _n in 0 .. 100 {
    //     let ds = py_matrix(ds_raw.clone());
    //     let s = py_matrix(s_raw.clone());
    //     s_raw = (ds.t() * s * ds).to_vec();
    //     ds_raw = gram_schmidt_raw(&s_raw);
    // }
}

fn proj(u: &Vec<f64>, v: &Vec<f64>) -> Vec<f64> {
    let uv = u.dot(v);
    let uu = u.dot(u);
    u.fmap(|x| x * uv / uu)
}

fn gram_schmidt(vs: &Matrix) -> Matrix {
    let mut result: Vec<Vec<f64>> = vec![vec![0f64; vs.col]; vs.row];
    result[0] = vs.col(0).clone();

    for k in 1 .. vs.col {
        let vk = &vs.col(k);
        let mut puv = vec![0f64; vk.len()];
        for j in 0 .. k {
            puv = puv.add(&proj(&result[j], vk));
        }
        result[k] = vk.sub(&puv);
    }

    py_matrix(
        result.into_iter().map(|v| {
            let n = v.norm();
            v.fmap(|t| t / n)
        }).collect()
    )
}

fn gram_schmidt_raw(vs: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
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

fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    match (a.row, a.col) {
        (p, q) if p <= 10 && q <= 10 => a * b,
        _ => {
            let (a1, a2, a3, a4) = a.block();
            let (b1, b2, b3, b4) = b.block();

            let m_list = vec![
                ((&a1, &b1), (&a2, &b3)),
                ((&a1, &b2), (&a2, &b4)),
                ((&a3, &b1), (&a4, &b3)),
                ((&a3, &b2), (&a4, &b4)),
            ];

            let m = m_list.into_par_iter().map(
                |(p, q)| p.0 * p.1 + q.0 * q.1
            ).collect::<Vec<Matrix>>();

            // let m1 = matmul(&a1,&b1) + matmul(&a2,&b3);
            // let m2 = matmul(&a1,&b2) + matmul(&a2,&b4);
            // let m3 = matmul(&a3,&b1) + matmul(&a4,&b3);
            // let m4 = matmul(&a3,&b2) + matmul(&a4,&b4);

            // combine(m1, m2, m3, m4)
            combine(m[0].clone(), m[1].clone(), m[2].clone(), m[3].clone())
        }
    }
}