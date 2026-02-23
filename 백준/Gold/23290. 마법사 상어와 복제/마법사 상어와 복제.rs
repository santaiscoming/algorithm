use std::{
    fs::File,
    io::{self, Read},
};

const F_DIRECTION: [(i32, i32); 8] =
    [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];
const S_DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (m, s) = next!(&mut tokens, usize, usize);

    let mut fishes = vec![vec![Vec::<usize>::new(); 4]; 4];
    for _ in 0..m {
        let (r, c, d) = next!(&mut tokens, usize, usize, usize);
        fishes[r - 1][c - 1].push(d - 1);
    }
    let (mut sr, mut sc) = next!(&mut tokens, usize, usize);
    sr -= 1;
    sc -= 1;
    let mut smells = vec![vec![0; 4]; 4];

    for _ in 0..s {
        // 1. 복제
        let snap = fishes.clone();

        // 2. 물고기 이동
        let mut new_fishes = vec![vec![Vec::<usize>::new(); 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                if !fishes[r][c].is_empty() {
                    let l = fishes[r][c].len();
                    for i in 0..l {
                        let mut d = fishes[r][c][i] as i32;
                        let (dr, dc) = F_DIRECTION[d as usize];

                        let mut nr = r.wrapping_add(dr as usize);
                        let mut nc = c.wrapping_add(dc as usize);

                        let origin = d;
                        let mut not_moved = false;
                        while !(nr < 4
                            && nc < 4
                            && smells[nr][nc] == 0
                            && !(nr == sr && nc == sc))
                        {
                            d = (d - 1 + 8) % 8;
                            if d == origin {
                                not_moved = true;
                                break;
                            }

                            let (dr, dc) = F_DIRECTION[d as usize];
                            nr = r.wrapping_add(dr as usize);
                            nc = c.wrapping_add(dc as usize);
                        }

                        let d = d as usize;
                        if not_moved {
                            new_fishes[r][c].push(fishes[r][c][i]);
                        } else {
                            new_fishes[nr][nc].push(d);
                        }
                    }
                }
            }
        }

        //3. 상어 이동
        let mut max = 0;
        let mut paths = Vec::<[usize; 3]>::new();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    let directions = [i, j, k];

                    let mut nr = sr;
                    let mut nc = sc;

                    let mut invalid = false;
                    let mut cnt = 0;
                    let mut visited = [(usize::MAX, usize::MAX); 3];
                    let mut visit_count = 0;

                    for d in directions {
                        let (dr, dc) = S_DIRECTION[d];

                        nr = nr.wrapping_add(dr as usize);
                        nc = nc.wrapping_add(dc as usize);
                        if nr >= 4 || nc >= 4 {
                            invalid = true;
                            break;
                        }
                        let already = (0..visit_count)
                            .any(|v| visited[v].0 == nr && visited[v].1 == nc);
                        if !already {
                            cnt += new_fishes[nr][nc].len();
                        }
                        visited[visit_count] = (nr, nc);
                        visit_count += 1;
                    }
                    if invalid {
                        continue;
                    } else {
                        if cnt > max {
                            max = cnt;
                            paths.clear();
                            paths.push([i, j, k]);
                        } else if cnt == max {
                            paths.push([i, j, k]);
                        }
                    }
                }
            }
        }

        paths.sort_unstable();
        let best = paths[0];
        for d in best {
            let (dr, dc) = S_DIRECTION[d];

            sr = sr.wrapping_add(dr as usize);
            sc = sc.wrapping_add(dc as usize);

            if !new_fishes[sr][sc].is_empty() {
                smells[sr][sc] = 3;
                new_fishes[sr][sc].clear();
            }
        }

        // 4. 냄세 제거
        for r in 0..4 {
            for c in 0..4 {
                if smells[r][c] > 0 {
                    smells[r][c] -= 1;
                }
            }
        }

        // 5. 복제
        for r in 0..4 {
            for c in 0..4 {
                if snap[r][c].len() > 0 {
                    new_fishes[r][c].extend(snap[r][c].iter());
                }
            }
        }

        fishes = new_fishes;
    }

    let f_ref = &fishes;
    let result = (0..4)
        .flat_map(move |r| (0..4).map(move |c| f_ref[r][c].len()))
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
