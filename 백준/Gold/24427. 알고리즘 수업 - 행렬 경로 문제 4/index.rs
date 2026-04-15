use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let mut grid = vec![vec![0i32; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            grid[i][j] = next!(&mut tokens, i32);
        }
    }
    let p = next!(&mut tokens, usize);

    let mut is_checkpoint = vec![vec![false; n + 1]; n + 1];
    for _ in 0..p {
        let r = next!(&mut tokens, usize);
        let c = next!(&mut tokens, usize);
        is_checkpoint[r][c] = true;
    }

    let mut dp = vec![vec![[0i32, 0i32]; n + 1]; n + 1];
    dp[1][1][0] = grid[1][1];
    for i in 1..=n {
        for j in 1..=n {
            if i == 1 && j == 1 {
                continue;
            }

            let prev_0 = max(dp[i - 1][j][0], dp[i][j - 1][0]);
            let prev_1 = max(dp[i - 1][j][1], dp[i][j - 1][1]);

            if is_checkpoint[i][j] {
                let max_prev = max(prev_0, prev_1);
                if max_prev > 0 {
                    dp[i][j][1] = max_prev + grid[i][j];
                }
            } else {
                if prev_0 > 0 {
                    dp[i][j][0] = prev_0 + grid[i][j];
                }
                if prev_1 > 0 {
                    dp[i][j][1] = prev_1 + grid[i][j];
                }
            }
        }
    }

    println!("{}", dp[n][n][1]);
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
