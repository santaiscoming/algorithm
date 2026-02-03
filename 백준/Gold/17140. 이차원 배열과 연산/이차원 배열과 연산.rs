use std::{
    fs::File,
    io::{self, Read},
};
fn main() {
    let mut tokens = read!();
    let (r, c, k) = next!(&mut tokens, usize, usize, usize);
    let (r, c) = (r - 1, c - 1);
    let board: Vec<Vec<usize>> = (0..3)
        .map(|_| {
            (0..3)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();

    let mut grid = [[0usize; 100]; 100];
    for r in 0..3 {
        for c in 0..3 {
            grid[r][c] = board[r][c];
        }
    }

    let mut row_len = 3;
    let mut col_len = 3;

    for time in 0..=100 {
        if r < 100 && c < 100 && grid[r][c] == k {
            println!("{}", time);
            return;
        }

        if row_len >= col_len {
            operate(&mut grid, &mut row_len, &mut col_len);
        } else {
            transpose(&mut grid, row_len, col_len);
            operate(&mut grid, &mut col_len, &mut row_len);
            transpose(&mut grid, col_len, row_len);
        }
    }

    println!("-1");
}

fn operate(
    grid: &mut [[usize; 100]; 100],
    r_len: &mut usize,
    c_len: &mut usize,
) {
    let mut max_len = 0;

    for r in 0..*r_len {
        let mut counts = [0u8; 101];
        for c in 0..*c_len {
            let val = grid[r][c];
            if val == 0 {
                continue;
            }
            counts[val] += 1;
        }

        let mut pairs: Vec<(u8, usize)> = Vec::with_capacity(100);
        for num in 1..=100 {
            if counts[num] > 0 {
                pairs.push((counts[num], num));
            }
        }

        pairs.sort_unstable();

        let mut idx = 0;
        for (count, num) in pairs {
            if idx >= 100 {
                break;
            }
            grid[r][idx] = num;
            grid[r][idx + 1] = count as usize;
            idx += 2;
        }

        max_len = max_len.max(idx);

        for i in idx..100 {
            grid[r][i] = 0;
        }
    }

    *c_len = max_len;
}

fn transpose(grid: &mut [[usize; 100]; 100], r_len: usize, c_len: usize) {
    let size = r_len.max(c_len);
    for r in 0..size {
        for c in (r + 1)..size {
            let temp = grid[r][c];
            grid[r][c] = grid[c][r];
            grid[c][r] = temp;
        }
    }
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
    ($tokens:expr) => {
        $tokens.next().unwrap()
    };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
