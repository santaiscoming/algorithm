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

    let mut tr = next!(&mut tokens, usize) - 1;
    let mut tc = next!(&mut tokens, usize) - 1;

    let mut customers = Vec::new();
    let mut customer_map = vec![vec![None; n]; n];
    for i in 0..m {
        let (sr, sc, dr, dc) = next!(&mut tokens, usize, usize, usize, usize);
        customers.push((sr - 1, sc - 1, dr - 1, dc - 1));
        customer_map[sr - 1][sc - 1] = Some(i);
    }

    for _ in 0..m {
        let target = find_customer(n, tr, tc, &grid, &customer_map);

        match target {
            None => {
                println!("-1");
                return;
            }
            Some((dist, idx)) => {
                if fuel < dist as i32 {
                    println!("-1");
                    return;
                }

                fuel -= dist as i32;
                tr = customers[idx].0;
                tc = customers[idx].1;
                customer_map[tr][tc] = None;

                let dest_r = customers[idx].2;
                let dest_c = customers[idx].3;

                let dist = get_dist(n, tr, tc, dest_r, dest_c, &grid);

                match dist {
                    None => {
                        println!("-1");
                        return;
                    }
                    Some(d) => {
                        if fuel < d as i32 {
                            println!("-1");
                            return;
                        }

                        fuel -= d as i32;
                        fuel += (d * 2) as i32;

                        tr = dest_r;
                        tc = dest_c;
                    }
                }
            }
        }
    }

    println!("{}", fuel);
}

fn find_customer(
    n: usize,
    sr: usize,
    sc: usize,
    grid: &Vec<Vec<usize>>,
    map: &Vec<Vec<Option<usize>>>,
) -> Option<(usize, usize)> {
    if let Some(idx) = map[sr][sc] {
        return Some((0, idx));
    }

    let mut visited = vec![vec![false; n]; n];
    let mut q = VecDeque::new();

    visited[sr][sc] = true;
    q.push_back((sr, sc, 0));

    let mut candidates = Vec::new();
    let mut min_dist = usize::MAX;

    while let Some((r, c, dist)) = q.pop_front() {
        if dist > min_dist {
            break;
        }

        for (dr, dc) in DIRECTIONS {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < n && nc < n {
                if !visited[nr][nc] && grid[nr][nc] != 1 {
                    visited[nr][nc] = true;

                    let next_dist = dist + 1;

                    if let Some(idx) = map[nr][nc] {
                        if next_dist <= min_dist {
                            min_dist = next_dist;
                            candidates.push((nr, nc, idx));
                        }
                    }

                    if next_dist < min_dist {
                        q.push_back((nr, nc, next_dist));
                    }
                }
            }
        }
    }

    if candidates.is_empty() {
        return None;
    }

    candidates.sort_unstable();

    Some((min_dist, candidates[0].2))
}

fn get_dist(
    n: usize,
    sr: usize,
    sc: usize,
    dr: usize,
    dc: usize,
    grid: &Vec<Vec<usize>>,
) -> Option<usize> {
    if sr == dr && sc == dc {
        return Some(0);
    }

    let mut visited = vec![vec![false; n]; n];
    let mut q = VecDeque::new();

    visited[sr][sc] = true;
    q.push_back((sr, sc, 0));

    while let Some((r, c, dist)) = q.pop_front() {
        if r == dr && c == dc {
            return Some(dist);
        }

        for (dr, dc) in DIRECTIONS {
            let nr = r.wrapping_add(dr as usize);
            let nc = c.wrapping_add(dc as usize);

            if nr < n && nc < n {
                let nr = nr as usize;
                let nc = nc as usize;

                if !visited[nr][nc] && grid[nr][nc] != 1 {
                    visited[nr][nc] = true;
                    q.push_back((nr, nc, dist + 1));
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
