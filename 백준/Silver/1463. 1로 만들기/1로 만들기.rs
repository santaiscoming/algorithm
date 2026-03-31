use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);

    let mut dp = vec![-1; n + 1];
    let res = recur(n, &mut dp);

    println!("{}", res);
}

fn recur(n: usize, dp: &mut Vec<i64>) -> i64 {
    if n <= 1 {
        return 0;
    }

    if dp[n] != -1 {
        return dp[n];
    }

    let mut ret = i64::MAX;
    if n % 3 == 0 {
        ret = ret.min(recur(n / 3, dp) + 1)
    }
    if n % 2 == 0 {
        ret = ret.min(recur(n / 2, dp) + 1)
    }
    ret = ret.min(recur(n - 1, dp) + 1);
    dp[n] = ret;

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
