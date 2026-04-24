use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let grid = (0..n)
        .map(|_| (0..n).map(|_| next!(&mut tokens, i32)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans = recur(n - 1, n - 1, n, &grid);
    println!("{}", ans);
}

fn recur(r: usize, c: usize, size: usize, grid: &Vec<Vec<i32>>) -> i32 {
    if size == 2 {
        let mut arr =
            [grid[r][c], grid[r - 1][c], grid[r][c - 1], grid[r - 1][c - 1]];
        arr.sort();
        return arr[2];
    }

    let half = size / 2;
    let mut arr = [
        recur(r, c, half, grid),
        recur(r - half, c, half, grid),
        recur(r, c - half, half, grid),
        recur(r - half, c - half, half, grid),
    ];
    arr.sort();

    arr[2]
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
