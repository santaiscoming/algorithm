use std::{
    fs::File,
    io::{self, Read},
    usize, vec,
};

const DUST_DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let mut tokens = read!();
    let (r, c, t) = next!(&mut tokens, usize, usize, usize);
    let mut board: Vec<Vec<i32>> = (0..r)
        .map(|_| {
            (0..c)
                .map(|_| next!(&mut tokens, i32))
                .collect()
        })
        .collect();

    let mut cleaners: Vec<usize> = Vec::new();
    for y in 0..r {
        if board[y][0] == -1 {
            cleaners.push(y);
        }
    }
    let cleaners = cleaners
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    for _ in 0..t {
        let mut new_board = vec![vec![0; c]; r];

        // 1. 확산
        for y in 0..r {
            for x in 0..c {
                if board[y][x] > 0 {
                    let mut candidate = Vec::new();
                    let mut cnt = 0;

                    for (dx, dy) in DUST_DIR {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0 && nx < c as i32 && ny >= 0 && ny < r as i32
                        {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            if board[ny][nx] == -1 {
                                continue;
                            }

                            candidate.push((nx, ny));
                            cnt += 1;
                        }
                    }

                    let cur = board[y][x];
                    let spread = cur / 5;
                    new_board[y][x] += cur - spread * cnt;
                    for (nx, ny) in candidate {
                        new_board[ny][nx] += spread;
                    }
                }
            }
        }

        // 2. 공기 청정
        let (ty, by) = cleaners[0];
        new_board[ty][0] = -1;
        new_board[by][0] = -1;

        let last_c = c - 1;

        for y in (1..ty).rev() {
            new_board[y][0] = new_board[y - 1][0];
        }
        new_board[0][0] = new_board[0][1];

        for x in 1..last_c {
            new_board[0][x] = new_board[0][x + 1];
        }
        new_board[0][last_c] = new_board[1][last_c];

        for y in 1..ty {
            new_board[y][last_c] = new_board[y + 1][last_c];
        }
        new_board[ty][last_c] = new_board[ty][last_c - 1];

        for x in (2..last_c).rev() {
            new_board[ty][x] = new_board[ty][x - 1];
        }
        new_board[ty][1] = 0;

        for y in by + 1..r - 1 {
            new_board[y][0] = new_board[y + 1][0];
        }
        new_board[r - 1][0] = new_board[r - 1][1];

        for x in 1..last_c {
            new_board[r - 1][x] = new_board[r - 1][x + 1];
        }
        new_board[r - 1][last_c] = new_board[r - 2][last_c];

        for y in (by + 1..r - 1).rev() {
            new_board[y][last_c] = new_board[y - 1][last_c];
        }
        new_board[by][last_c] = new_board[by][last_c - 1];

        for x in (2..last_c).rev() {
            new_board[by][x] = new_board[by][x - 1];
        }
        new_board[by][1] = 0;

        board = new_board;
    }

    let result: i32 = board
        .iter()
        .flat_map(|v| v.iter())
        .filter(|&&x| x > 0)
        .sum();
    println!("{result}");
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
