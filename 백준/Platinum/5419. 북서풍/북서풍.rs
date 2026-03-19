use std::{
    cmp::Reverse,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);

    for _ in 0..t {
        let n = next!(&mut tokens, usize);
        let mut points: Vec<(i64, i64)> =
            (0..n).map(|_| next!(&mut tokens, i64, i64)).collect();

        let mut ys: Vec<i64> = points.iter().map(|&(_, y)| y).collect();
        ys.sort();
        ys.dedup();
        let compress_y = |y: i64| -> usize {
            let pos = ys.binary_search(&y).unwrap();
            ys.len() - pos
        };

        points.sort_by_key(|&v| (v.0, Reverse(v.1)));

        let m = ys.len();
        let mut bit = vec![0i64; m + 1];
        let mut answer: i64 = 0;

        for &(_, y) in &points {
            let cy = compress_y(y);

            answer += bit_query(&bit, cy);

            bit_update(&mut bit, cy, 1, m);
        }

        println!("{}", answer);
    }
}

fn bit_update(bit: &mut [i64], mut i: usize, val: i64, n: usize) {
    while i <= n {
        bit[i] += val;
        i += i & i.wrapping_neg();
    }
}

fn bit_query(bit: &[i64], mut i: usize) -> i64 {
    let mut sum = 0;
    while i > 0 {
        sum += bit[i];
        i -= i & i.wrapping_neg();
    }
    sum
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
