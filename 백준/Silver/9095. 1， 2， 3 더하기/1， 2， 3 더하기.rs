use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);
    let tcs = (0..t)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<_>>();

    for n in tcs {
        let mut dp = vec![-1; n + 1];
        let res = recur(n as i32, &mut dp);

        println!("{}", res);
    }
}

fn recur(n: i32, dp: &mut Vec<i32>) -> i32 {
    if n == 0 {
        return 1;
    }
    if n < 0 {
        return 0;
    }

    if dp[n as usize] != -1 {
        return dp[n as usize];
    }

    let ret = recur(n - 1, dp) + recur(n - 2, dp) + recur(n - 3, dp);
    dp[n as usize] = ret;

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
