use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (n, m, mut fuel) = next!(&mut tokens, usize, usize, i32);
    let grid: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let (mut cr, mut cc) = next!(&mut tokens, usize, usize);
    cr -= 1;
    cc -= 1;
    let mut customers: Vec<Option<(usize, usize, usize, usize)>> = (0..m)
        .map(|_| {
            let (r, c, tr, tc) = next!(&mut tokens, usize, usize, usize, usize);
            Some((r - 1, c - 1, tr - 1, tc - 1))
        })
        .collect();

    for _ in 0..m {
        match bfs(cr, cc, &mut fuel, &grid, &mut customers) {
            Some((nr, nc)) => {
                cr = nr;
                cc = nc;
            }
            None => {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", fuel);
}

fn bfs(
    sr: usize,
    sc: usize,
    fuel: &mut i32,
    grid: &Vec<Vec<usize>>,
    customers: &mut Vec<Option<(usize, usize, usize, usize)>>,
) -> Option<(usize, usize)> {
    let n = grid.len();

    let mut dist = vec![vec![-1i32; n]; n];
    dist[sr][sc] = 0;

    let mut q = VecDeque::new();
    q.push_back((sr, sc));

    let mut candidates: Vec<(usize, usize, usize)> = Vec::new();
    let mut found_dist = -1i32;

    while let Some((cr, cc)) = q.pop_front() {
        let cur_dist = dist[cr][cc];

        if found_dist >= 0 && cur_dist > found_dist {
            break;
        }

        for (i, c) in customers.iter().enumerate() {
            if let Some((r, col, _, _)) = c {
                if *r == cr && *col == cc {
                    candidates.push((*r, *col, i));
                    found_dist = cur_dist;
                }
            }
        }

        if found_dist >= 0 {
            continue;
        }

        for (dr, dc) in DIRECTIONS {
            let nr = cr as i32 + dr;
            let nc = cc as i32 + dc;

            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;

                if dist[nr][nc] == -1 && grid[nr][nc] == 0 {
                    dist[nr][nc] = cur_dist + 1;
                    q.push_back((nr, nc));
                }
            }
        }
    }

    if candidates.is_empty() {
        return None;
    }

    candidates.sort();
    let (_, _, idx) = candidates[0];

    let customer = customers[idx].unwrap();
    let (pr, pc, tr, tc) = customer;

    *fuel -= found_dist;
    if *fuel < 0 {
        return None;
    }

    customers[idx] = None;

    let mut dist2 = vec![vec![-1i32; n]; n];
    dist2[pr][pc] = 0;

    let mut q2 = VecDeque::new();
    q2.push_back((pr, pc));

    while let Some((cr, cc)) = q2.pop_front() {
        let cur_dist = dist2[cr][cc];

        if cr == tr && cc == tc {
            *fuel -= cur_dist;
            if *fuel < 0 {
                return None;
            }
            *fuel += cur_dist * 2;
            return Some((tr, tc));
        }

        for (dr, dc) in DIRECTIONS {
            let nr = cr as i32 + dr;
            let nc = cc as i32 + dc;

            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                let nr = nr as usize;
                let nc = nc as usize;

                if dist2[nr][nc] == -1 && grid[nr][nc] == 0 {
                    dist2[nr][nc] = cur_dist + 1;
                    q2.push_back((nr, nc));
                }
            }
        }
    }

    None
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
