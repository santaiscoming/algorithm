use std::{
    fs::File,
    io::{self, Read},
};

const MOD: i64 = 1_000_000_007;
fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let k = next!(&mut tokens, usize);

    let mut dp = vec![0i64; k + 1];
    dp[0] = 1;

    for i in 1..=n {
        let r = i.min(k);
        for j in (1..=r).rev() {
            dp[j] += dp[j - 1];
            if dp[j] >= MOD {
                dp[j] -= MOD;
            }
        }
    }

    let mut ans = dp[k];
    for _ in 0..k - 1 {
        ans = ans * 2 % MOD;
    }

    println!("{ans}");
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
