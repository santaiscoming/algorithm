use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let mut board: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();

    let (mut sx, mut sy) = (0, 0);
    for y in 0..n {
        for x in 0..n {
            if board[y][x] == 9 {
                sx = x;
                sy = y;
                board[y][x] = 0;
            }
        }
    }

    let mut sec = 0;
    let mut level = 2;
    let mut xp = 0;

    loop {
        match find_fish(&board, sx, sy, level, n) {
            Some((dist, ny, nx)) => {
                sec += dist;
                board[ny][nx] = 0;
                sx = nx;
                sy = ny;

                xp += 1;
                if xp == level {
                    level += 1;
                    xp = 0;
                }
            }
            None => break,
        }
    }

    println!("{}", sec);
}

fn find_fish(
    board: &Vec<Vec<usize>>,
    sx: usize,
    sy: usize,
    level: usize,
    n: usize,
) -> Option<(usize, usize, usize)> {
    let mut visited = vec![vec![false; n]; n];
    let mut pq = BinaryHeap::new();

    pq.push(Reverse((0, sy, sx)));

    while let Some(Reverse((dist, y, x))) = pq.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if board[y][x] > 0 && board[y][x] < level {
            return Some((dist, y, x));
        }

        for (dy, dx) in DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if ny >= 0 && nx >= 0 && ny < n as i32 && nx < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited[ny][nx] && board[ny][nx] <= level {
                    pq.push(Reverse((dist + 1, ny, nx)));
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
