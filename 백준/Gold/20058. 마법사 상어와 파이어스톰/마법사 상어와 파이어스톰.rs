use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
type Grid = Vec<Vec<usize>>;

fn main() {
    let mut tokens = read!();
    let (n, q) = next!(&mut tokens, usize, usize);
    let mut grid: Grid = (0..1 << n)
        .map(|_| {
            (0..1 << n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let steps = (0..q)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<_>>();

    for step in steps {
        grid = rotate(1 << step, &mut grid);
        reconcile(&mut grid)
    }

    let n = grid.len();

    let mut ice = 0;
    let mut max_island = 0;
    let mut visited = vec![vec![false; n]; n];

    for r in 0..n {
        for c in 0..n {
            ice += grid[r][c];

            if !visited[r][c] && grid[r][c] > 0 {
                visited[r][c] = true;

                let cnt = bfs(r, c, &grid, &mut visited);
                if cnt > max_island {
                    max_island = cnt;
                }
            }
        }
    }

    println!("{}", ice);
    println!("{}", max_island);
}

fn bfs(r: usize, c: usize, grid: &Grid, visited: &mut Vec<Vec<bool>>) -> usize {
    let n = grid.len();
    let mut cnt = 1;

    let mut q = VecDeque::new();
    q.push_back((r, c));

    while let Some((r, c)) = q.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < n && nc < n && !visited[nr][nc] && grid[nr][nc] > 0 {
                visited[nr][nc] = true;
                cnt += 1;
                q.push_back((nr, nc));
            }
        }
    }

    cnt
}

fn reconcile(grid: &mut Grid) {
    let n = grid.len();
    let snap = grid.clone();

    for r in 0..n {
        for c in 0..n {
            if snap[r][c] == 0 {
                continue;
            }

            let mut ice_neighbors = 0;
            for (dr, dc) in DIRECTIONS {
                let nr = r.wrapping_add(dr as usize);
                let nc = c.wrapping_add(dc as usize);

                if nr < n && nc < n && snap[nr][nc] > 0 {
                    ice_neighbors += 1;
                }
            }

            if ice_neighbors < 3 {
                grid[r][c] -= 1;
            }
        }
    }
}

fn rotate(count: usize, grid: &mut Grid) -> Grid {
    fn _rotate(
        sr: usize,
        sc: usize,
        count: usize,
        origin: &Grid,
        new: &mut Grid,
    ) {
        for r in 0..count {
            for c in 0..count {
                new[sr + r][sc + c] = origin[sr + count - 1 - c][sc + r];
            }
        }
    }

    let n = grid.len();
    let mut new_grid = vec![vec![0; n]; n];

    for r in (0..grid.len()).step_by(count) {
        for c in (0..grid.len()).step_by(count) {
            _rotate(r, c, count, grid, &mut new_grid);
        }
    }

    new_grid
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
