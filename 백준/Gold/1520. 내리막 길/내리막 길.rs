use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let grid: Vec<Vec<usize>> = (0..n)
        .map(|_| (0..m).map(|_| next!(&mut tokens, usize)).collect())
        .collect();

    let mut dp = vec![vec![-1; m]; n];
    let res = recur(0, 0, &grid, &mut dp);
    println!("{}", res)
}

fn recur(
    r: usize,
    c: usize,
    grid: &Vec<Vec<usize>>,
    dp: &mut Vec<Vec<i64>>,
) -> i64 {
    let n = grid.len();
    let m = grid[0].len();
    if r == n - 1 && c == m - 1 {
        return 1;
    }

    if dp[r][c] != -1 {
        return dp[r][c];
    }

    let mut ret = 0;
    for (dr, dc) in DIRECTION {
        let nr = r.wrapping_add(dr as usize);
        let nc = c.wrapping_add(dc as usize);
        if nr < n && nc < m && grid[r][c] > grid[nr][nc] {
            ret += recur(nr, nc, grid, dp);
        }
    }

    dp[r][c] = ret;
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
