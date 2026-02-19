use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 8] =
    [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];

type Grid = Vec<Vec<usize>>;

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let mut grid: Grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let moves: Vec<(usize, usize)> = (0..m)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect();

    let mut clouds: Vec<(usize, usize)> = Vec::new();
    for (r, c) in [(n - 1, 0), (n - 1, 1), (n - 2, 0), (n - 2, 1)] {
        clouds.push((r, c));
    }

    for (d, s) in moves {
        // 1. 이동
        let mut moved_clouds = Vec::new();
        for &(r, c) in &clouds {
            let (dr, dc) = DIRECTIONS[d - 1];
            let nr = (r as i32 + dr * s as i32).rem_euclid(n as i32) as usize;
            let nc = (c as i32 + dc * s as i32).rem_euclid(n as i32) as usize;

            moved_clouds.push((nr, nc));
        }

        // 2. 증가
        for &(r, c) in &moved_clouds {
            grid[r][c] += 1;
        }

        // 4. 물복사
        for &(r, c) in &moved_clouds {
            let mut cnt = 0;
            for d in (1..8).step_by(2) {
                let (dr, dc) = DIRECTIONS[d];
                let nr = r.wrapping_add(dr as usize);
                let nc = c.wrapping_add(dc as usize);

                if nr < n && nc < n && grid[nr][nc] > 0 {
                    cnt += 1;
                }
            }

            grid[r][c] += cnt;
        }

        // 5. 구름 생성
        let mut new_cloud = Vec::new();
        for r in 0..n {
            for c in 0..n {
                if grid[r][c] >= 2 && !moved_clouds.contains(&(r, c)) {
                    new_cloud.push((r, c));
                    grid[r][c] -= 2;
                }
            }
        }

        clouds = new_cloud;
    }

    let result: usize = grid.iter().flat_map(|r| r.iter()).sum();
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
