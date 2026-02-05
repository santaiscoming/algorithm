use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let board: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();

    let total: usize = board.iter().flatten().sum();
    let mut result = usize::MAX;

    for x in 0..n {
        for y in 0..n {
            for d1 in 1..n {
                for d2 in 1..n {
                    if x + d1 + d2 >= n || y < d1 || y + d2 >= n {
                        continue;
                    }

                    let mut is_five = vec![vec![false; n]; n];
                    for r in 0..n {
                        for c in 0..n {
                            if r + c >= x + y
                                && r + c <= x + y + 2 * d2
                                && (r as isize - c as isize)
                                    >= (x as isize - y as isize)
                                && (r as isize - c as isize)
                                    <= (x as isize - y as isize
                                        + 2 * d1 as isize)
                            {
                                is_five[r][c] = true;
                            }
                        }
                    }

                    let mut counts = [0; 5];

                    for r in 0..x + d1 {
                        for c in 0..=y {
                            if is_five[r][c] {
                                break;
                            }
                            counts[0] += board[r][c];
                        }
                    }

                    for r in 0..=x + d2 {
                        for c in (y + 1..n).rev() {
                            if is_five[r][c] {
                                break;
                            }
                            counts[1] += board[r][c];
                        }
                    }

                    for r in x + d1..n {
                        for c in 0..y - d1 + d2 {
                            if is_five[r][c] {
                                break;
                            }
                            counts[2] += board[r][c];
                        }
                    }

                    for r in x + d2 + 1..n {
                        for c in (y - d1 + d2..n).rev() {
                            if is_five[r][c] {
                                break;
                            }
                            counts[3] += board[r][c];
                        }
                    }

                    counts[4] =
                        total - counts[0] - counts[1] - counts[2] - counts[3];

                    let min = counts.iter().min().unwrap();
                    let max = counts.iter().max().unwrap();
                    result = result.min(max - min);
                }
            }
        }
    }

    println!("{}", result);
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
