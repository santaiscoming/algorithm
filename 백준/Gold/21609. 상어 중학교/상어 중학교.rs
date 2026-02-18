use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const EMPTY: i32 = -2;
const BLACK: i32 = -1;
const RAINBOW: i32 = 0;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Grid = Vec<Vec<i32>>;

fn main() {
    let mut tokens = read!();
    let (n, _m) = next!(&mut tokens, usize, usize);
    let mut grid: Grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, i32))
                .collect()
        })
        .collect();

    let mut score = 0;

    loop {
        match find_largest_group(&grid) {
            None => break,
            Some((cells, _)) => {
                let cnt = cells.len();
                score += (cnt * cnt) as i32;
                for (r, c) in &cells {
                    grid[*r][*c] = EMPTY;
                }
            }
        }

        gravity(&mut grid);
        rotate(&mut grid);
        gravity(&mut grid);
    }

    println!("{}", score);
}

fn bfs_group(
    sr: usize,
    sc: usize,
    grid: &Grid,
    visited: &mut Vec<Vec<bool>>,
) -> Option<(Vec<(usize, usize)>, usize, (usize, usize))> {
    let n = grid.len();
    let color = grid[sr][sc];

    if color <= 0 || color == EMPTY {
        return None;
    }
    if visited[sr][sc] {
        return None;
    }

    let mut q = VecDeque::new();
    let mut cells = Vec::new();
    let mut rainbow = Vec::new();

    let mut l_visited = vec![vec![false; n]; n];

    q.push_back((sr, sc));
    visited[sr][sc] = true;
    l_visited[sr][sc] = true;

    while let Some((r, c)) = q.pop_front() {
        cells.push((r, c));
        if grid[r][c] == RAINBOW {
            rainbow.push((r, c));
        }

        for (dr, dc) in DIRECTIONS {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < n && nc < n {
                let nr = nr as usize;
                let nc = nc as usize;
                if l_visited[nr][nc] {
                    continue;
                }
                let v = grid[nr][nc];

                if v == color || v == RAINBOW {
                    l_visited[nr][nc] = true;
                    if v == color {
                        visited[nr][nc] = true;
                    }
                    q.push_back((nr, nc));
                }
            }
        }
    }

    let cnt = cells
        .iter()
        .filter(|&&(r, c)| grid[r][c] != RAINBOW)
        .count();
    if cnt < 1 {
        return None;
    }

    if cells.len() < 2 {
        return None;
    }

    let rainbow_cnt = rainbow.len();

    let base = cells
        .iter()
        .filter(|&&(r, c)| grid[r][c] != RAINBOW)
        .min()
        .copied()
        .unwrap();

    Some((cells, rainbow_cnt, base))
}

fn find_largest_group(
    grid: &Grid,
) -> Option<(Vec<(usize, usize)>, (usize, usize))> {
    let n = grid.len();
    let mut visited = vec![vec![false; n]; n];
    let mut best: Option<(usize, usize, (usize, usize), Vec<(usize, usize)>)> =
        None;

    for r in 0..n {
        for c in 0..n {
            if let Some((cells, rainbow_cnt, base)) =
                bfs_group(r, c, grid, &mut visited)
            {
                let key = (cells.len(), rainbow_cnt, base.0, base.1);
                let candidate = match &best {
                    None => true,
                    Some((sz, rc, base_pos, _)) => {
                        if key.0 != *sz {
                            key.0 > *sz
                        } else if key.1 != *rc {
                            key.1 > *rc
                        } else if key.2 != base_pos.0 {
                            key.2 > base_pos.0
                        } else {
                            key.3 > base_pos.1
                        }
                    }
                };

                if candidate {
                    best = Some((key.0, key.1, (key.2, key.3), cells));
                }
            }
        }
    }

    best.map(|(_, _, base, cells)| (cells, base))
}

fn gravity(grid: &mut Grid) {
    let n = grid.len();
    for c in 0..n {
        let mut empty_r = n as i32 - 1;

        while empty_r >= 0 && grid[empty_r as usize][c] != EMPTY {
            empty_r -= 1;
        }

        let mut cur = empty_r - 1;
        while cur >= 0 {
            let cv = grid[cur as usize][c];
            if cv == BLACK {
                empty_r = cur - 1;

                while empty_r >= 0 && grid[empty_r as usize][c] != EMPTY {
                    empty_r -= 1;
                }

                cur = empty_r - 1;
            } else if cv != EMPTY {
                grid[empty_r as usize][c] = cv;
                grid[cur as usize][c] = EMPTY;
                empty_r -= 1;

                while empty_r >= 0 && grid[empty_r as usize][c] != EMPTY {
                    empty_r -= 1;
                }

                cur -= 1;
            } else {
                cur -= 1;
            }
        }
    }
}

fn rotate(grid: &mut Grid) {
    let n = grid.len();
    let mut new_grid = vec![vec![0i32; n]; n];

    for r in 0..n {
        for c in 0..n {
            new_grid[n - 1 - c][r] = grid[r][c];
        }
    }

    *grid = new_grid;
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
