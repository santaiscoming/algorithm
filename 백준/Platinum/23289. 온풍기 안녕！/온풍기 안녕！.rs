use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

type Grid = Vec<Vec<i64>>;

fn main() {
    let mut tokens = read!();
    let (n, m, k) = next!(&mut tokens, usize, usize, usize);
    let grid: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..m)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let w = next!(&mut tokens, usize);
    let input_walls: Vec<(usize, usize, usize)> = (0..w)
        .map(|_| next!(&mut tokens, usize, usize, usize))
        .collect();

    let mut wall_up = vec![vec![false; m]; n];
    let mut wall_right = vec![vec![false; m]; n];
    for &(x, y, t) in &input_walls {
        let (x, y) = (x - 1, y - 1);
        if t == 0 {
            wall_up[x][y] = true;
        } else {
            wall_right[x][y] = true;
        }
    }
    let mut room: Grid = vec![vec![0; m]; n];
    let heaters: Vec<(usize, usize, usize)> = (0..n)
        .flat_map(move |rr| (0..m).map(move |cc| (rr, cc)))
        .filter(|&(r, c)| grid[r][c] >= 1 && grid[r][c] <= 4)
        .map(|(r, c)| (r, c, grid[r][c]))
        .collect();
    let tests: Vec<(usize, usize)> = (0..n)
        .flat_map(move |rr| (0..m).map(move |cc| (rr, cc)))
        .filter(|&(r, c)| grid[r][c] == 5)
        .collect();
    let mut choco = 0;

    loop {
        // 1. 온풍기 ON
        for &(hr, hc, d) in &heaters {
            blow(&mut room, n, m, hr, hc, d, &wall_up, &wall_right);
        }

        // 2. 온도 조절
        adjust(&mut room, n, m, &wall_up, &wall_right);

        // 3. 바깥쪽 온도 1 감소
        decrease(&mut room, n, m);

        // 4. 냠냠
        choco += 1;

        // 5. 조사
        if tests
            .iter()
            .all(|&(r, c)| room[r][c] >= k as i64)
        {
            break;
        }
        if choco >= 101 {
            break;
        }
    }

    println!("{}", choco);
}

fn has_wall(
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
    wall_up: &Vec<Vec<bool>>,
    wall_right: &Vec<Vec<bool>>,
) -> bool {
    if r1 == r2 {
        if c2 == c1 + 1 {
            wall_right[r1][c1]
        } else if c1 == c2 + 1 {
            wall_right[r1][c2]
        } else {
            false
        }
    } else if c1 == c2 {
        if r2 == r1 + 1 {
            wall_up[r2][c1]
        } else if r1 == r2 + 1 {
            wall_up[r1][c1]
        } else {
            false
        }
    } else {
        false
    }
}

fn blow(
    room: &mut Grid,
    n: usize,
    m: usize,
    hr: usize,
    hc: usize,
    d: usize,
    wall_up: &Vec<Vec<bool>>,
    wall_right: &Vec<Vec<bool>>,
) {
    let (dr, dc): (i32, i32) = match d {
        1 => (0, 1),
        2 => (0, -1),
        3 => (-1, 0),
        4 => (1, 0),
        _ => unreachable!(),
    };

    let sr = hr as i32 + dr;
    let sc = hc as i32 + dc;
    if sr < 0 || sr >= n as i32 || sc < 0 || sc >= m as i32 {
        return;
    }
    if has_wall(hr, hc, sr as usize, sc as usize, wall_up, wall_right) {
        return;
    }

    let mut visited = vec![vec![false; m]; n];
    let (sr, sc) = (sr as usize, sc as usize);
    visited[sr][sc] = true;
    room[sr][sc] += 5;

    let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();
    queue.push_back((sr, sc, 5));

    let diags: [(i32, i32, i32, i32); 2] = match d {
        1 => [(-1, 0, -1, 1), (1, 0, 1, 1)],
        2 => [(-1, 0, -1, -1), (1, 0, 1, -1)],
        3 => [(0, -1, -1, -1), (0, 1, -1, 1)],
        4 => [(0, -1, 1, -1), (0, 1, 1, 1)],
        _ => unreachable!(),
    };

    while let Some((cr, cc, temp)) = queue.pop_front() {
        if temp <= 1 {
            continue;
        }
        let nt = temp - 1;

        // 직진
        let (nr, nc) = (cr as i32 + dr, cc as i32 + dc);
        if nr >= 0 && nr < n as i32 && nc >= 0 && nc < m as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if !visited[nr][nc]
                && !has_wall(cr, cc, nr, nc, wall_up, wall_right)
            {
                visited[nr][nc] = true;
                room[nr][nc] += nt;
                queue.push_back((nr, nc, nt));
            }
        }

        // 대각선 2방향
        for &(mdr, mdc, fdr, fdc) in &diags {
            let (mr, mc) = (cr as i32 + mdr, cc as i32 + mdc);
            let (fr, fc) = (cr as i32 + fdr, cc as i32 + fdc);
            if mr < 0 || mr >= n as i32 || mc < 0 || mc >= m as i32 {
                continue;
            }
            if fr < 0 || fr >= n as i32 || fc < 0 || fc >= m as i32 {
                continue;
            }
            let (mr, mc, fr, fc) =
                (mr as usize, mc as usize, fr as usize, fc as usize);
            if !visited[fr][fc]
                && !has_wall(cr, cc, mr, mc, wall_up, wall_right)
                && !has_wall(mr, mc, fr, fc, wall_up, wall_right)
            {
                visited[fr][fc] = true;
                room[fr][fc] += nt;
                queue.push_back((fr, fc, nt));
            }
        }
    }
}

fn adjust(
    room: &mut Grid,
    n: usize,
    m: usize,
    wall_up: &Vec<Vec<bool>>,
    wall_right: &Vec<Vec<bool>>,
) {
    let mut delta = vec![vec![0i64; m]; n];
    for i in 0..n {
        for j in 0..m {
            if j + 1 < m && !wall_right[i][j] {
                let d = (room[i][j] - room[i][j + 1]).abs() / 4;
                if room[i][j] > room[i][j + 1] {
                    delta[i][j] -= d;
                    delta[i][j + 1] += d;
                } else {
                    delta[i][j] += d;
                    delta[i][j + 1] -= d;
                }
            }
            if i + 1 < n && !wall_up[i + 1][j] {
                let d = (room[i][j] - room[i + 1][j]).abs() / 4;
                if room[i][j] > room[i + 1][j] {
                    delta[i][j] -= d;
                    delta[i + 1][j] += d;
                } else {
                    delta[i][j] += d;
                    delta[i + 1][j] -= d;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            room[i][j] += delta[i][j];
        }
    }
}

fn decrease(room: &mut Grid, n: usize, m: usize) {
    for j in 0..m {
        if room[0][j] > 0 {
            room[0][j] -= 1;
        }
    }
    for j in 0..m {
        if room[n - 1][j] > 0 {
            room[n - 1][j] -= 1;
        }
    }
    for i in 1..n - 1 {
        if room[i][0] > 0 {
            room[i][0] -= 1;
        }
    }
    for i in 1..n - 1 {
        if room[i][m - 1] > 0 {
            room[i][m - 1] -= 1;
        }
    }
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
