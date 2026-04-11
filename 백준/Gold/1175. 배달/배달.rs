use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let grid = (0..n)
        .map(|_| next!(&mut tokens, String).chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut s = (0, 0);
    let mut targets = Vec::new();
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == 'S' {
                s = (r, c);
            } else if grid[r][c] == 'C' {
                targets.push((r, c));
            }
        }
    }

    let (sr, sc) = s;
    let mut visited = vec![vec![vec![vec![false; 4]; 5]; m]; n];
    let mut q = VecDeque::new();

    q.push_back((sr, sc, 4, 0, 0));
    visited[sr][sc][4][0] = true;

    while let Some((r, c, prev, arrived_bit, steps)) = q.pop_front() {
        if arrived_bit == 3 {
            println!("{}", steps);
            return;
        }

        for d in 0..4 {
            if d != prev {
                let (dr, dc) = DIRECTIONS[d];
                let nr = r.wrapping_add(dr as usize);
                let nc = c.wrapping_add(dc as usize);
                if nr < n && nc < m && grid[nr][nc] != '#' {
                    let mut next = arrived_bit;

                    for i in 0..2 {
                        if targets[i] == (nr, nc) {
                            let bit = if i == 0 { 1 } else { 2 };
                            next |= bit
                        }
                    }

                    if !visited[nr][nc][d][next] {
                        visited[nr][nc][d][next] = true;
                        q.push_back((nr, nc, d, next, steps + 1));
                    }
                }
            }
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
