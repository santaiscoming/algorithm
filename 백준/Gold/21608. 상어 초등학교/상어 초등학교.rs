use std::{
    cmp::Reverse,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let students: Vec<(usize, [usize; 4])> = (0..n * n)
        .map(|_| {
            let (id, a, b, c, d) =
                next!(&mut tokens, usize, usize, usize, usize, usize);
            (id, [a, b, c, d])
        })
        .collect();

    // 1. 배치
    let mut grid: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for &(id, prefer) in &students {
        let mut candidate: Vec<(usize, usize, usize, usize)> = Vec::new();

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] > 0 {
                    continue;
                }

                let mut empty = 0;
                let mut perfer_cnt = 0;
                for dr in -1i32..=1 {
                    for dc in -1i32..=1 {
                        let nr = r.wrapping_add(dr as usize);
                        let nc = c.wrapping_add(dc as usize);
                        let r = r as i32;
                        let c = c as i32;

                        if nr < n
                            && nc < n
                            && (r - nr as i32).abs() + (c - nc as i32).abs()
                                == 1
                        {
                            if prefer.contains(&grid[nr][nc]) {
                                perfer_cnt += 1;
                            }
                            if grid[nr][nc] == 0 {
                                empty += 1
                            }
                        }
                    }
                }

                candidate.push((perfer_cnt, empty, r, c));
            }
        }

        candidate.sort_by_key(|&(a, b, c, d)| (Reverse(a), Reverse(b), c, d));
        let (_, _, r, c) = candidate[0];
        grid[r][c] = id;
    }

    // 2. 만족도
    let mut result = 0;
    for r in 0..n {
        for c in 0..n {
            let id = grid[r][c];
            let (_, prefer) = &students
                .iter()
                .find(|&v| v.0 == id)
                .unwrap();

            let mut cnt = 0;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    let nr = r.wrapping_add(dr as usize);
                    let nc = c.wrapping_add(dc as usize);
                    let r = r as i32;
                    let c = c as i32;

                    if nr < n
                        && nc < n
                        && (r - nr as i32).abs() + (c - nc as i32).abs() == 1
                        && prefer.contains(&grid[nr][nc])
                    {
                        cnt += 1;
                    }
                }
            }

            result += if cnt == 0 {
                0
            } else {
                10usize.pow(cnt - 1)
            }
        }
    }

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
