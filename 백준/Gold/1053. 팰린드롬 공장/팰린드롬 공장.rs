use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let mut s: Vec<char> = next!(&mut tokens, String).chars().collect();

    let n = s.len();
    let mut dp = vec![vec![-1; n]; n];
    let mut res = recur(0, n - 1, &s, &mut dp);

    for i in 0..n {
        for j in i + 1..n {
            if s[i] != s[j] {
                s.swap(i, j);
                let mut dp = vec![vec![-1; n]; n];
                res = res.min(recur(0, n - 1, &s, &mut dp) + 1);
                s.swap(i, j);
            }
        }
    }

    println!("{}", res);
}

fn recur(l: usize, r: usize, s: &Vec<char>, dp: &mut Vec<Vec<i64>>) -> i64 {
    if l >= r {
        return 0;
    }

    if dp[l][r] != -1 {
        return dp[l][r];
    }

    let left = recur(l + 1, r, s, dp) + 1;
    let right = recur(l, r - 1, s, dp) + 1;
    let both = recur(l + 1, r - 1, s, dp) + if s[l] == s[r] { 0 } else { 1 };

    dp[l][r] = left.min(right).min(both);

    dp[l][r]
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
