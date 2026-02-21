use std::{
    cmp,
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

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

    let mut result = 0;
    let mut d = 0;
    let mut cr: usize = 0;
    let mut cc: usize = 0;
    let mut dice = [0, 1, 2, 3, 4, 5, 6].to_vec();

    for _ in 0..k {
        // 1. 주사위 움직임
        let (dr, dc) = DIRECTIONS[d];
        let mut nr = cr.wrapping_add(dr as usize);
        let mut nc = cc.wrapping_add(dc as usize);

        if nr >= n || nc >= m {
            d = (d + 2) % 4;
            let (dr, dc) = DIRECTIONS[d];
            nr = cr.wrapping_add(dr as usize);
            nc = cc.wrapping_add(dc as usize);
        }
        cr = nr;
        cc = nc;
        turn(&mut dice, d);

        // 2. 도착한 칸 점수 획득
        let b = grid[cr][cc];
        let cnt = score_bfs(cr, cc, &grid);

        result += b * cnt;

        // 3. 이동 방향 결정
        let a = dice[6];
        if a > b {
            d = (d + 1) % 4
        } else if a < b {
            d = (d + 3) % 4
        }
    }

    println!("{}", result);
}

fn score_bfs(sr: usize, sc: usize, grid: &Vec<Vec<usize>>) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let snap = grid[sr][sc];
    let mut visited = vec![vec![false; m]; n];
    let mut q = VecDeque::new();

    visited[sr][sc] = true;
    q.push_back((sr, sc));

    let mut result = 1;

    while let Some((cr, cc)) = q.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let nr = cr.wrapping_add(dr as usize);
            let nc = cc.wrapping_add(dc as usize);

            if nr < n && nc < m && !visited[nr][nc] && grid[nr][nc] == snap {
                visited[nr][nc] = true;
                q.push_back((nr, nc));
                result += 1;
            }
        }
    }

    result
}

fn turn(dice: &mut Vec<usize>, d: usize) {
    match d {
        0 => {
            let tmp = dice[3];
            dice[3] = dice[1];
            dice[1] = dice[4];
            dice[4] = dice[6];
            dice[6] = tmp;
        }
        1 => {
            let tmp = dice[6];
            dice[6] = dice[5];
            dice[5] = dice[1];
            dice[1] = dice[2];
            dice[2] = tmp;
        }
        2 => {
            let tmp = dice[4];
            dice[4] = dice[1];
            dice[1] = dice[3];
            dice[3] = dice[6];
            dice[6] = tmp;
        }
        3 => {
            let tmp = dice[2];
            dice[2] = dice[1];
            dice[1] = dice[5];
            dice[5] = dice[6];
            dice[6] = tmp;
        }
        _ => unreachable!(),
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
