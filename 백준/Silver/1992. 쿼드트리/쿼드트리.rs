use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!().into_iter();
    let n = next!(&mut tokens, usize);
    let grid = (0..n)
        .map(|_| next!(&mut tokens, String).chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans = recur(n - 1, n - 1, n, &grid);
    println!("{}", ans)
}

fn recur(r: usize, c: usize, size: usize, grid: &Vec<Vec<char>>) -> String {
    if size == 1 {
        return grid[r][c].to_string();
    }

    if size == 2 {
        let arr =
            [grid[r - 1][c - 1], grid[r - 1][c], grid[r][c - 1], grid[r][c]];
        if arr.iter().all(|&v| v == '0') {
            return "0".to_string();
        } else if arr.iter().all(|&v| v == '1') {
            return "1".to_string();
        } else {
            let s = [
                grid[r - 1][c - 1],
                grid[r - 1][c],
                grid[r][c - 1],
                grid[r][c],
            ]
            .iter()
            .collect::<String>();

            return format!("({s})");
        }
    }

    let half = size / 2;
    let arr = [
        recur(r - half, c - half, half, grid),
        recur(r - half, c, half, grid),
        recur(r, c - half, half, grid),
        recur(r, c, half, grid),
    ];
    if arr.iter().all(|v| v == "0") {
        return "0".to_string();
    } else if arr.iter().all(|v| v == "1") {
        return "1".to_string();
    }
    let s = arr.join("");

    format!("({s})")
}

#[macro_export]
macro_rules! read {
    () => {{
        let mut buf = String::new();
        match File::open("input.txt") {
            Ok(mut f) => f.read_to_string(&mut buf).unwrap(),
            Err(_) => io::stdin().read_to_string(&mut buf).unwrap(),
        };
        buf.split_ascii_whitespace()
            .map(str::to_owned)
            .collect::<Vec<String>>()
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
