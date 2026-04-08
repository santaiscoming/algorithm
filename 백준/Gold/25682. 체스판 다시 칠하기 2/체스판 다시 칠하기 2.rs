use std::{
    fs::File,
    io::{self, Read},
    vec,
};

fn main() {
    let mut tokens = read!();
    let (n, m, k) = next!(&mut tokens, usize, usize, usize);
    let grid: Vec<Vec<_>> = (0..n)
        .map(|_| next!(&mut tokens, String).chars().collect())
        .collect();

    let mut paint_cnt = vec![vec![0; m]; n];
    for r in 0..n {
        for c in 0..m {
            let expect_white = (r + c) % 2 == 0;
            if (expect_white && grid[r][c] == 'B')
                || (!expect_white && grid[r][c] == 'W')
            {
                paint_cnt[r][c] = 1;
            }
        }
    }

    let mut prefix = vec![vec![0; m + 1]; n + 1];
    for r in 1..=n {
        for c in 1..=m {
            prefix[r][c] = prefix[r - 1][c] + prefix[r][c - 1]
                - prefix[r - 1][c - 1]
                + paint_cnt[r - 1][c - 1];
        }
    }

    let mut res = usize::MAX;
    for i in k..=n {
        for j in k..=m {
            let a = prefix[i][j] - prefix[i - k][j] - prefix[i][j - k]
                + prefix[i - k][j - k];
            let comp = k * k - a;

            res = res.min(a).min(comp)
        }
    }

    println!("{}", res);
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
