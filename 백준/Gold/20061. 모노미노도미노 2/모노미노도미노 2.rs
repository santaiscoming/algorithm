use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(tokens, usize);
    let blocks: Vec<(usize, usize, usize)> = (0..n)
        .map(|_| next!(tokens, usize, usize, usize))
        .collect();

    let mut green = [[false; 4]; 6];
    let mut blue = [[false; 4]; 6];

    let mut total_score = 0;

    for (t, x, y) in blocks {
        total_score += simulation(&mut green, t, y);

        let t_blue = match t {
            1 => 1,
            2 => 3,
            3 => 2,
            _ => unreachable!(),
        };
        total_score += simulation(&mut blue, t_blue, x);
    }

    println!("{}", total_score);
    println!("{}", count_blocks(&green) + count_blocks(&blue));
}

fn simulation(board: &mut [[bool; 4]; 6], t: usize, c: usize) -> u32 {
    let mut score = 0;

    let mut r = 0;

    if t == 1 {
        while r + 1 < 6 && !board[r + 1][c] {
            r += 1;
        }
        board[r][c] = true;
    } else if t == 2 {
        while r + 1 < 6 && !board[r + 1][c] && !board[r + 1][c + 1] {
            r += 1;
        }
        board[r][c] = true;
        board[r][c + 1] = true;
    } else if t == 3 {
        while r + 1 < 6 && !board[r + 1][c] {
            r += 1;
        }
        board[r][c] = true;
        board[r - 1][c] = true;
    }

    let mut new_board = [[false; 4]; 6];
    let mut target_r = 5;

    for row in (0..6).rev() {
        let is_full = board[row].iter().all(|&x| x);

        if is_full {
            score += 1;
        } else {
            new_board[target_r] = board[row];
            if target_r > 0 {
                target_r -= 1;
            }
        }
    }
    *board = new_board;

    let mut push_count = 0;
    if board[0].iter().any(|&x| x) {
        push_count = 2;
    } else if board[1].iter().any(|&x| x) {
        push_count = 1;
    }

    if push_count > 0 {
        let mut special_board = [[false; 4]; 6];
        for row in 0..(6 - push_count) {
            special_board[row + push_count] = board[row];
        }
        *board = special_board;
    }

    score
}

fn count_blocks(board: &[[bool; 4]; 6]) -> usize {
    board
        .iter()
        .flatten()
        .filter(|&&x| x)
        .count()
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
