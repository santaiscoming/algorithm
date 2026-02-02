use std::{
    fs::File,
    io::{self, Read},
    usize, vec,
};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
fn main() {
    let mut tokens = read!();
    let (r, c, m) = next!(&mut tokens, usize, usize, usize);
    let mut board: Vec<Vec<Option<(usize, usize, usize)>>> =
        vec![vec![None; c]; r];
    for _ in 0..m {
        let (r, c, s, d, z) =
            next!(&mut tokens, usize, usize, usize, usize, usize);
        board[r - 1][c - 1] = Some((s, d - 1, z));
    }

    let mut result = 0;

    for pos in 0..c {
        // 1. 잡고
        for y in 0..r {
            if let Some(v) = board[y][pos] {
                let (_, _, z) = v;
                board[y][pos] = None;
                result += z;
                break;
            }
        }

        // 2. 상어 이동
        let mut new_board: Vec<Vec<Option<(usize, usize, usize)>>> =
            vec![vec![None; c]; r];
        for y in 0..r {
            for x in 0..c {
                if let Some(v) = board[y][x] {
                    let (s, d, z) = v;
                    let mut shark = move_shark((x, y, s, d, z), r, c);
                    let (nx, ny, _, _, _) = shark;

                    if let Some(prev_shark) = new_board[ny][nx] {
                        let (ps, pd, pz) = prev_shark;

                        shark = if shark.4 > pz {
                            shark
                        } else {
                            (nx, ny, ps, pd, pz)
                        }
                    }

                    let (nx, ny, s, d, z) = shark;
                    new_board[ny][nx] = Some((s, d, z));
                }
            }
        }
        board = new_board
    }

    println!("{}", result);
}

fn move_shark(
    shark: (usize, usize, usize, usize, usize),
    r: usize,
    c: usize,
) -> (usize, usize, usize, usize, usize) {
    let (mut x, mut y, s, mut d, z) = shark;
    let moves = if d < 2 {
        s % (2 * (r - 1))
    } else {
        s % (2 * (c - 1))
    };

    for _ in 0..moves {
        let (dx, dy) = DIRECTIONS[d];
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if ny < 0 || ny >= r as i32 || nx < 0 || nx >= c as i32 {
            d = d ^ 1;
        }

        let (dx, dy) = DIRECTIONS[d];
        x = (x as i32 + dx) as usize;
        y = (y as i32 + dy) as usize;
    }
    (x, y, s, d, z)
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
