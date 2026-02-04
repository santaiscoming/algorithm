use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let mut board = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut virus_axis = Vec::new();
    for y in 0..n {
        for x in 0..n {
            if board[y][x] == 2 {
                virus_axis.push((x, y));
            }
        }
    }

    let mut virus_combi: Vec<Vec<(usize, usize)>> = Vec::new();
    combi(&virus_axis, m, 0, 0, &mut virus_combi, &mut Vec::new());

    let mut result = None;
    for virus in virus_combi {
        let candidate = bfs(&mut board.clone(), &virus);
        if let Some(sec) = candidate {
            result = Some(result.unwrap_or(sec).min(sec));
        }
    }

    println!("{:?}", result.unwrap_or(-1));
}

fn combi(
    arr: &Vec<(usize, usize)>,
    cnt: usize,
    s: usize,
    depth: usize,
    result: &mut Vec<Vec<(usize, usize)>>,
    temp: &mut Vec<(usize, usize)>,
) {
    if cnt == depth {
        result.push(temp.clone());
        return;
    }

    for i in s..arr.len() {
        temp.push(arr[i]);
        combi(arr, cnt, i + 1, depth + 1, result, temp);
        temp.pop();
    }
}

fn bfs(
    board: &mut Vec<Vec<usize>>,
    virus: &Vec<(usize, usize)>,
) -> Option<i32> {
    let mut empty_count: i32 = board
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&v| v == 0)
        .count() as i32;

    if empty_count == 0 {
        return Some(0);
    }

    let n = board.len();

    let mut sec = 0;

    let mut visited = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    for &(x, y) in virus {
        visited[y][x] = true;
        q.push_front((x, y));
    }

    loop {
        if q.is_empty() {
            break;
        }

        let mut new_virus = Vec::new();
        while let Some((x, y)) = q.pop_front() {
            for (dx, dy) in DIRECTIONS {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);

                if nx < n
                    && ny < n
                    && !visited[ny][nx]
                    && (board[ny][nx] == 0 || board[ny][nx] == 2)
                {
                    visited[ny][nx] = true;
                    if board[ny][nx] == 0 {
                        empty_count -= 1;
                    }
                    board[ny][nx] = 3;
                    new_virus.push((nx, ny));
                }
            }
        }

        if !new_virus.is_empty() {
            sec += 1;
        }
        if empty_count == 0 {
            return Some(sec);
        }

        for v in new_virus {
            q.push_front(v);
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
