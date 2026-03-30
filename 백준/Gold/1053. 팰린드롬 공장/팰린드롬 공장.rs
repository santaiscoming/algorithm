use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let mut s: Vec<char> = next!(&mut tokens, String).chars().collect();

    let mut res = d_p(&s);
    let n = s.len();

    for a in 0..n {
        for b in (a + 1)..n {
            if s[a] != s[b] {
                s.swap(a, b);
                res = res.min(d_p(&s) + 1);
                s.swap(a, b);
            }
        }
    }

    println!("{}", res);
}

fn d_p(s: &Vec<char>) -> usize {
    let n = s.len();
    if n <= 1 {
        return 0;
    }

    let mut dp = vec![vec![0usize; n]; n];

    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if s[i] == s[j] {
                dp[i][j] = if len == 2 { 0 } else { dp[i + 1][j - 1] };
            } else {
                let del_left = dp[i + 1][j];
                let del_right = dp[i][j - 1];
                let replace = if len == 2 { 0 } else { dp[i + 1][j - 1] };
                dp[i][j] = del_left.min(del_right).min(replace) + 1;
            }
        }
    }
    dp[0][n - 1]
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
