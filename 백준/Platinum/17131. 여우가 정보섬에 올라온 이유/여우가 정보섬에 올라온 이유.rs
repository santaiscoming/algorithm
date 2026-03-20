use std::{
    cmp::Reverse,
    fs::File,
    io::{self, Read},
};

const MOD: i64 = 1_000_000_007;

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let mut stars: Vec<(i64, i64)> =
        (0..n).map(|_| next!(&mut tokens, i64, i64)).collect();

    let mut xs: Vec<i64> = stars.iter().map(|&(x, _)| x).collect();
    xs.sort();
    xs.dedup();
    let m = xs.len();

    stars.sort_by_key(|&(x, y)| (Reverse(y), x));

    let mut bit = vec![0i64; m + 1];
    let mut answer: i64 = 0;
    let mut i = 0;

    while i < n {
        let cur_y = stars[i].1;

        let mut j = i;
        while j < n && stars[j].1 == cur_y {
            j += 1;
        }

        for k in i..j {
            let cx = xs.partition_point(|&v| v < stars[k].0) + 1;

            let left = if cx > 1 { bit_query(&bit, cx - 1) } else { 0 };
            let right = bit_query(&bit, m) - bit_query(&bit, cx);
            answer = (answer + (left % MOD) * (right % MOD)) % MOD;
        }

        for k in i..j {
            let cx = xs.partition_point(|&v| v < stars[k].0) + 1;
            bit_update(&mut bit, cx, 1, m);
        }

        i = j;
    }

    println!("{}", answer);
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
