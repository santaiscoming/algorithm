use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut dp = vec![vec![-1; n]; n];
    let res = recur(0, n - 1, &arr, &mut dp);
    println!("{}", res);
}

fn recur(l: usize, r: usize, arr: &Vec<usize>, dp: &mut Vec<Vec<i32>>) -> i32 {
    if l >= r {
        return 0;
    }

    if dp[l][r] != -1 {
        return dp[l][r];
    }

    let mut ret = i32::MAX;
    if arr[l] != arr[r] {
        let left = recur(l, r - 1, arr, dp);
        let right = recur(l + 1, r, arr, dp);

        ret = ret.min(left.min(right) + 1)
    } else {
        ret = ret.min(recur(l + 1, r - 1, arr, dp))
    }

    dp[l][r] = ret;

    ret
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
