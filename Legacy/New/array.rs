fn main() {
    let mut v: Vec<i64> = vec![0; 1000000];
    
    let mut iter = G { val: 1 };
    
    let mut i = 0usize;

    while i < v.len() {
        v[i] = iter.next().unwrap();
        i += 1;
    }

    let mut s = 0i64;
    for val in v.into_iter() {
        s += val;
    }
    
    println!("{}", (s as f64) / (1000000 as f64));
}

struct G {
    val: i64
}

impl Iterator for G {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let old = self.val;
        self.val += 2;
        Some(old)
    }
}
