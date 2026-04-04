use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let a = next!(&mut tokens, String);
    let b = next!(&mut tokens, String);

    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![-1; m + 1]; n + 1];
    let res = recur(
        0,
        0,
        &a.chars().collect::<Vec<_>>(),
        &b.chars().collect::<Vec<_>>(),
        &mut dp,
    );
    println!("{}", res);
}

fn recur(
    i: usize,
    j: usize,
    a: &Vec<char>,
    b: &Vec<char>,
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    let n = a.len();
    let m = b.len();
    if i == n {
        return (m - j) as i32;
    }
    if j == m {
        return (n - i) as i32;
    }

    if dp[i][j] != -1 {
        return dp[i][j];
    }

    let mut ret = 0i32;
    if a[i] == b[j] {
        ret = recur(i + 1, j + 1, a, b, dp);
    } else {
        let ins = recur(i + 1, j, a, b, dp);
        let del = recur(i, j + 1, a, b, dp);
        let rep = recur(i + 1, j + 1, a, b, dp);

        ret = ins.min(del).min(rep) + 1;
    }
    dp[i][j] = ret;

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
