use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let arr: Vec<_> =
        (0..n).map(|_| next!(&mut tokens, usize, usize)).collect();

    let mut upper = 0;
    let mut sweep = vec![0i32; 1_000_001];
    for i in 0..n {
        let (s, e) = arr[i];
        sweep[s] += 1;
        sweep[e] -= 1;
        upper = upper.max(e);
    }

    let mut acc = 0;
    let mut prefix = vec![0usize; upper + 1];
    for i in 0..upper + 1 {
        acc += sweep[i];
        prefix[i] = acc as usize;
    }

    let mut l = 0;
    let mut sum = 0;
    for r in 0..upper + 1 {
        if sum == k {
            println!("{} {}", l, r);
            return;
        }
        sum += prefix[r];

        while sum > k {
            sum -= prefix[l];
            l += 1;
        }
    }

    println!("0 0");
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
