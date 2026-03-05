use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let rects = (0..n)
        .map(|_| {
            let (x, y, w, h) = next!(&mut tokens, f64, f64, f64, f64);
            (
                (x * 10.0) as i32,
                (y * 10.0) as i32,
                (w * 10.0) as i32,
                (h * 10.0) as i32,
            )
        })
        .collect::<Vec<_>>();

    let mut r_coord = Vec::new();
    let mut c_coord = Vec::new();
    for &(c, r, w, h) in &rects {
        let r1 = r;
        let r2 = r + h;
        let c1 = c;
        let c2 = c + w;

        [r1, r2].iter().for_each(|&v| r_coord.push(v));
        [c1, c2].iter().for_each(|&v| c_coord.push(v));
    }
    r_coord.sort();
    c_coord.sort();
    r_coord.dedup();
    c_coord.dedup();

    let mut result = 0;
    for i in 0..r_coord.len() - 1 {
        for j in 0..c_coord.len() - 1 {
            let r1 = r_coord[i];
            let r2 = r_coord[i + 1];
            let c1 = c_coord[j];
            let c2 = c_coord[j + 1];

            for &rect in &rects {
                if [
                    c1 >= rect.0,
                    c2 <= rect.0 + rect.2,
                    r1 >= rect.1,
                    r2 <= rect.1 + rect.3,
                ]
                .iter()
                .all(|&v| v)
                {
                    result += (r2 - r1) * (c2 - c1);
                    break;
                }
            }
        }
    }

    if result % 100 == 0 {
        println!("{}", result / 100);
    } else {
        println!("{:.2}", result as f64 / 100.0);
    }
}

#[macro_export]
macro_rules! read {
    () => {{
        let mut buf = String::new();
        match File::open("input.txt") {
            Ok(mut f) => f.read_to_string(&mut buf).unwrap(),
            Err(_) => io::stdin().read_to_string(&mut buf).unwrap(),
        };
        Box::leak(buf.into_boxed_str()).split_ascii_whitespace()
    }};
}

#[macro_export]
macro_rules! next {
    ($tokens:expr) => {
        $tokens.next().unwrap()
    };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
