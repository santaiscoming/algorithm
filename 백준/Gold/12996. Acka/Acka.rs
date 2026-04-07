use std::{
    fs::File,
    io::{self, Read},
};

const MOD: i32 = 1_000_000_007;

fn main() {
    let mut tokens = read!();
    let s = next!(&mut tokens, usize);
    let a = next!(&mut tokens, usize);
    let b = next!(&mut tokens, usize);
    let c = next!(&mut tokens, usize);

    let mut memo = vec![vec![vec![vec![-1; 51]; 51]; 51]; 51];
    let ans = dp(s, a, b, c, &mut memo);

    println!("{}", ans);
}

fn dp(
    s: usize,
    a: usize,
    b: usize,
    c: usize,
    memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
) -> i32 {
    if s == 0 {
        if a == 0 && b == 0 && c == 0 {
            return 1;
        }
        return 0;
    }
    if a > s || b > s || c > s {
        return 0;
    }

    if memo[s][a][b][c] != -1 {
        return memo[s][a][b][c];
    }

    let mut ret = 0;
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }

                if a >= i && b >= j && c >= k {
                    ret = (ret + dp(s - 1, a - i, b - j, c - k, memo)) % MOD;
                }
            }
        }
    }

    memo[s][a][b][c] = ret;
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
