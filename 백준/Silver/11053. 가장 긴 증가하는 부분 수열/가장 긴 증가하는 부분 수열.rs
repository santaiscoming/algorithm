use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr = (0..n)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<_>>();

    let mut dp = vec![1; n];
    for i in 1..n {
        let cur = arr[i];
        for j in (0..=i - 1).rev() {
            let prev = arr[j];

            if cur > prev {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }
    
    println!("{}", dp.iter().max().unwrap());
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
