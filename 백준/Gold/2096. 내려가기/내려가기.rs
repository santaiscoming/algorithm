use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr: Vec<_> = (0..n)
        .map(|_| {
            let (a, b, c) = next!(&mut tokens, usize, usize, usize);
            [a, b, c]
        })
        .collect();

    let mut max_dp = arr[0];
    let mut min_dp = arr[0];

    for i in 1..n {
        let [r0, r1, r2] = arr[i];

        let new_max = [
            r0 + max_dp[0].max(max_dp[1]),
            r1 + max_dp[0].max(max_dp[1]).max(max_dp[2]),
            r2 + max_dp[1].max(max_dp[2]),
        ];
        let new_min = [
            r0 + min_dp[0].min(min_dp[1]),
            r1 + min_dp[0].min(min_dp[1]).min(min_dp[2]),
            r2 + min_dp[1].min(min_dp[2]),
        ];

        max_dp = new_max;
        min_dp = new_min;
    }

    let max_result = max_dp[0].max(max_dp[1]).max(max_dp[2]);
    let min_result = min_dp[0].min(min_dp[1]).min(min_dp[2]);
    println!("{} {}", max_result, min_result);
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
