use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 8] =
    [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)];

type Board = [[Option<(usize, usize)>; 4]; 4];

fn main() {
    let mut tokens = read!();
    let mut board: Board = [[None; 4]; 4];

    for r in 0..4 {
        for c in 0..4 {
            let (num, d) = next!(tokens, usize, usize);
            board[r][c] = Some((num, d - 1));
        }
    }

    let (num, d) = board[0][0].unwrap();
    board[0][0] = None;

    let result = num + dfs(0, 0, d, &board);
    println!("{}", result);
}

fn dfs(sx: usize, sy: usize, sd: usize, board: &Board) -> usize {
    let mut board = *board;

    for fish_num in 1..=16 {
        let mut found = None;
        for r in 0..4 {
            for c in 0..4 {
                if let Some((num, d)) = board[r][c] {
                    if num == fish_num {
                        found = Some((c, r, d));
                    }
                }
            }
        }

        let (fx, fy, fd) = match found {
            Some(v) => v,
            None => continue,
        };

        let mut nd = fd;
        for _ in 0..8 {
            let (dy, dx) = DIRECTIONS[nd];
            let nx = fx.wrapping_add(dx as usize);
            let ny = fy.wrapping_add(dy as usize);

            if nx >= 4 || ny >= 4 {
                nd = (nd + 1) % 8;
                continue;
            }

            if nx == sx && ny == sy {
                nd = (nd + 1) % 8;
                continue;
            }

            let target = board[ny][nx];
            board[ny][nx] = Some((fish_num, nd));
            board[fy][fx] = target;
            break;
        }
    }

    let mut max_score = 0;
    let (dy, dx) = DIRECTIONS[sd];
    let mut cx = sx;
    let mut cy = sy;

    for _ in 0..3 {
        cx = cx.wrapping_add(dx as usize);
        cy = cy.wrapping_add(dy as usize);

        if cx >= 4 || cy >= 4 {
            break;
        }

        if let Some((num, fd)) = board[cy][cx] {
            let mut next_board = board;
            next_board[cy][cx] = None;

            let score = num + dfs(cx, cy, fd, &next_board);
            max_score = max_score.max(score);
        }
    }

    max_score
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
