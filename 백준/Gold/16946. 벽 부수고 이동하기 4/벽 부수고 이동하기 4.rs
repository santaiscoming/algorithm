use std::{
    fs::File,
    io::{self, Read},
};

const DRIECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let grid = (0..n)
        .map(|_| {
            next!(&mut tokens, String)
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut id = 1;
    let mut group_id = vec![vec![0; m]; n];
    let mut group_cnt = vec![0];
    let mut visited = vec![vec![false; m]; n];

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == 0 && !visited[r][c] {
                visited[r][c] = true;
                group_id[r][c] = id;

                let cnt = dfs(r, c, 0, id, &mut visited, &mut group_id, &grid);
                group_cnt.push(cnt);
                id += 1;
            }
        }
    }

    let mut ans = grid.clone();
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == 1 {
                let mut visited_id = vec![false; group_cnt.len()];
                let mut acc = 1;

                for (dr, dc) in DRIECTIONS {
                    let nr = r.wrapping_add(dr as usize);
                    let nc = c.wrapping_add(dc as usize);
                    if nr < n && nc < m {
                        let id = group_id[nr][nc];
                        if !visited_id[id] {
                            acc += group_cnt[id];
                            visited_id[id] = true
                        }
                    }
                }

                ans[r][c] = acc;
            }
        }
    }
    for r in ans {
        let line: String =
            r.into_iter().map(|v| (v % 10).to_string()).collect();
        println!("{}", line);
    }
}

fn dfs(
    r: usize,
    c: usize,
    cnt: usize,
    id: usize,
    visited: &mut Vec<Vec<bool>>,
    group: &mut Vec<Vec<usize>>,
    grid: &Vec<Vec<usize>>,
) -> usize {
    let n = grid.len();
    let m = grid[0].len();

    let mut ret = 1;
    for (dr, dc) in DRIECTIONS {
        let nr = r.wrapping_add(dr as usize);
        let nc = c.wrapping_add(dc as usize);

        if nr < n && nc < m && grid[nr][nc] == 0 && !visited[nr][nc] {
            visited[nr][nc] = true;
            group[nr][nc] = id;
            ret += dfs(nr, nc, cnt + 1, id, visited, group, grid);
        }
    }

    return ret;
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
