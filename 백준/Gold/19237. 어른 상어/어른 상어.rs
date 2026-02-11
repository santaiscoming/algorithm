use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (n, m, k) = next!(&mut tokens, usize, usize, usize);

    let mut grid = vec![vec![0usize; n]; n];
    for r in 0..n {
        for c in 0..n {
            grid[r][c] = next!(&mut tokens, usize);
        }
    }

    let init_dirs: Vec<usize> = (0..m)
        .map(|_| next!(&mut tokens, usize) - 1)
        .collect();
    let priority: Vec<Vec<[usize; 4]>> = (0..m)
        .map(|_| {
            (0..4)
                .map(|_| {
                    let (a, b, c, d) =
                        next!(&mut tokens, usize, usize, usize, usize);
                    [a - 1, b - 1, c - 1, d - 1]
                })
                .collect()
        })
        .collect();

    let mut sharks: Vec<Option<(usize, usize, usize)>> = vec![None; m];
    for r in 0..n {
        for c in 0..n {
            if grid[r][c] > 0 {
                let id = grid[r][c] - 1;
                sharks[id] = Some((r, c, init_dirs[id]));
            }
        }
    }

    let mut smell: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); n]; n];
    for id in 0..m {
        if let Some((r, c, _)) = sharks[id] {
            smell[r][c] = (id, k);
        }
    }

    let mut sec = 0;
    while sec < 1000 {
        let mut new_sharks: Vec<Option<(usize, usize, usize)>> = vec![None; m];

        for id in 0..m {
            let (cr, cc, cd) = match sharks[id] {
                Some(v) => v,
                None => continue,
            };

            let mut moved = false;
            for pri_dir in priority[id][cd] {
                let (dr, dc) = DIRECTIONS[pri_dir];
                let nr = cr as i32 + dr;
                let nc = cc as i32 + dc;
                if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if smell[nr][nc].1 == 0 {
                        new_sharks[id] = Some((nr, nc, pri_dir));
                        moved = true;
                        break;
                    }
                }
            }

            if !moved {
                for pri_dir in priority[id][cd] {
                    let (dr, dc) = DIRECTIONS[pri_dir];
                    let nr = cr as i32 + dr;
                    let nc = cc as i32 + dc;
                    if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if smell[nr][nc].0 == id && smell[nr][nc].1 > 0 {
                            new_sharks[id] = Some((nr, nc, pri_dir));
                            break;
                        }
                    }
                }
            }
        }

        sharks = new_sharks;
        for i in 0..m {
            if sharks[i].is_none() {
                continue;
            }
            let (ri, ci, _) = sharks[i].unwrap();
            for j in (i + 1)..m {
                if let Some((rj, cj, _)) = sharks[j] {
                    if ri == rj && ci == cj {
                        sharks[j] = None;
                    }
                }
            }
        }

        for r in 0..n {
            for c in 0..n {
                if smell[r][c].1 > 0 {
                    smell[r][c].1 -= 1;
                }
            }
        }

        for id in 0..m {
            if let Some((r, c, _)) = sharks[id] {
                smell[r][c] = (id, k);
            }
        }

        sec += 1;

        let alive_count = sharks
            .iter()
            .filter(|s| s.is_some())
            .count();
        if alive_count == 1 {
            println!("{}", sec);
            return;
        }
    }

    println!("-1");
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
