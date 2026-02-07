use std::{
    fs::File,
    io::{self, Read},
    usize,
};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (n, m, t) = next!(&mut tokens, usize, usize, usize);
    let mut board: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..m)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let tc: Vec<(usize, usize, usize)> = (0..t)
        .map(|_| next!(&mut tokens, usize, usize, usize))
        .collect();

    for (x, d, k) in tc {
        for row in 0..n {
            if (row + 1) % x == 0 {
                for _ in 0..k {
                    turn(&mut board, row, d);
                }
            }
        }

        let mut axis: Vec<(usize, usize)> = Vec::new();

        for row in 0..n {
            for col in 0..m {
                let cur = board[row][col];
                if cur == 0 {
                    continue;
                }

                let left = (col + m - 1) % m;
                let right = (col + 1) % m;

                if board[row][left] == cur {
                    axis.push((col, row));
                    axis.push((left, row));
                }
                if board[row][right] == cur {
                    axis.push((col, row));
                    axis.push((right, row));
                }

                if row > 0 && board[row - 1][col] == cur {
                    axis.push((col, row));
                    axis.push((col, row - 1));
                }
                if row < n - 1 && board[row + 1][col] == cur {
                    axis.push((col, row));
                    axis.push((col, row + 1));
                }
            }
        }

        if !axis.is_empty() {
            for (x, y) in axis {
                board[y][x] = 0;
            }
        } else {
            let total: usize = board
                .iter()
                .flatten()
                .filter(|&&v| v > 0)
                .sum();
            let count = board
                .iter()
                .flatten()
                .filter(|&&v| v > 0)
                .count();

            if count > 0 {
                let avr = total as f64 / count as f64;

                for row in &mut board {
                    for v in row {
                        if *v > 0 {
                            if (*v as f64) > avr {
                                *v -= 1;
                            } else if (*v as f64) < avr {
                                *v += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    let result = board.iter().flatten().sum::<usize>();
    println!("{result}");
}

fn turn(board: &mut Vec<Vec<usize>>, row: usize, direction: usize) {
    let m = board[row].len();

    if direction == 0 {
        board[row].rotate_right(1);
    } else {
        board[row].rotate_left(1);
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
