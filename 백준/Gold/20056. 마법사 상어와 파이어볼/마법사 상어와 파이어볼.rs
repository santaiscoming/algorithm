use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 8] =
    [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
type Grid = Vec<Vec<Vec<(usize, usize, usize)>>>;

fn main() {
    let mut tokens = read!();
    let (n, m, k) = next!(&mut tokens, usize, usize, usize);

    let mut grid: Grid = vec![vec![Vec::new(); n]; n];
    for _ in 0..m {
        let (r, c, m, s, d) =
            next!(&mut tokens, usize, usize, usize, usize, usize);
        grid[r - 1][c - 1].push((m, s, d));
    }

    for _ in 0..k {
        let mut new_grid: Grid = vec![vec![Vec::new(); n]; n];

        for r in 0..n {
            for c in 0..n {
                let cur = &grid[r][c];
                let l = cur.len();

                for i in 0..l {
                    let (m, s, d) = cur[i];

                    let (dr, dc) = DIRECTIONS[d];
                    let n = n as i32;
                    let nr = (((r as i32 + (dr * s as i32)) % n) + n) % n;
                    let nc = (((c as i32 + (dc * s as i32)) % n) + n) % n;

                    new_grid[nr as usize][nc as usize].push((m, s, d));
                }
            }
        }

        for r in 0..n {
            for c in 0..n {
                if new_grid[r][c].len() >= 2 {
                    let l = new_grid[r][c].len();

                    let new_weight = new_grid[r][c]
                        .iter()
                        .map(|&v| v.0)
                        .sum::<usize>()
                        / 5;
                    if new_weight < 1 {
                        new_grid[r][c].clear();
                        continue;
                    }

                    let new_speed = new_grid[r][c]
                        .iter()
                        .map(|&v| v.1)
                        .sum::<usize>()
                        / l;

                    let dirs = new_grid[r][c]
                        .iter()
                        .map(|&v| v.2)
                        .collect::<Vec<_>>();

                    let new_directions: [usize; 4] =
                        if dirs.iter().all(|v| v % 2 == 0)
                            || dirs.iter().all(|v| v % 2 == 1)
                        {
                            [0, 2, 4, 6]
                        } else {
                            [1, 3, 5, 7]
                        };

                    new_grid[r][c] = new_directions
                        .iter()
                        .map(|&nd| (new_weight, new_speed, nd))
                        .collect::<Vec<_>>();
                }
            }
        }

        grid = new_grid
    }

    let result = grid
        .iter()
        .flatten()
        .flatten()
        .map(|&v| v.0)
        .sum::<usize>();
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
