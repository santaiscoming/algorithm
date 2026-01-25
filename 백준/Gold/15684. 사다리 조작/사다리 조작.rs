use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m, h) = next!(&mut tokens, usize, usize, usize);
    let infos = (0..m)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    let mut ladder = vec![vec![false; n + 1]; h + 1];
    for &(a, b) in &infos {
        ladder[a][b] = true;
    }

    for cnt in 0..=3 {
        if dfs(&mut ladder, n, h, cnt, 1, 1) {
            println!("{}", cnt);
            return;
        }
    }
    println!("-1");
}

fn check(ladder: &Vec<Vec<bool>>, n: usize, h: usize) -> bool {
    for start in 1..=n {
        let mut pos = start;
        for row in 1..=h {
            if pos < n && ladder[row][pos] {
                pos += 1;
            } else if pos > 1 && ladder[row][pos - 1] {
                pos -= 1;
            }
        }
        if pos != start {
            return false;
        }
    }
    true
}

fn dfs(
    ladder: &mut Vec<Vec<bool>>,
    n: usize,
    h: usize,
    remain: usize,
    start_row: usize,
    start_col: usize,
) -> bool {
    if remain == 0 {
        return check(ladder, n, h);
    }

    for row in start_row..=h {
        let col_start = if row == start_row {
            start_col
        } else {
            1
        };
        for col in col_start..n {
            if ladder[row][col] {
                continue;
            }
            if col > 1 && ladder[row][col - 1] {
                continue;
            }
            if col < n - 1 && ladder[row][col + 1] {
                continue;
            }

            ladder[row][col] = true;
            if dfs(ladder, n, h, remain - 1, row, col + 2) {
                return true;
            }

            ladder[row][col] = false;
        }
    }
    false
}

#[macro_export]
macro_rules! read {
    () => {{
        let mut buf = String::new();
        match File::open("input.txt") {
            Ok(mut f) => f.read_to_string(&mut buf).unwrap(),
            Err(_) => io::stdin()
                .read_to_string(&mut buf)
                .unwrap(),
        };
        Box::leak(buf.into_boxed_str()).split_ascii_whitespace()
    }};
}

#[macro_export]
macro_rules! next {
    ($tokens:expr) => { $tokens.next().unwrap() };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
