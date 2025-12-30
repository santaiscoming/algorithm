use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let (n, m, board) = get_inputs();

    let mut red = (0, 0);
    let mut blue = (0, 0);
    let mut hole = (0, 0);
    for i in 0..n {
        for j in 0..m {
            match board[i][j] {
                'R' => red = (i as i32, j as i32),
                'B' => blue = (i as i32, j as i32),
                'O' => hole = (i as i32, j as i32),
                _ => {}
            }
        }
    }
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let s = State {
        red,
        blue,
        moves: 0,
    };
    q.push_back(s);
    visited.insert((red, blue));

    while let Some(s) = q.pop_front() {
        if s.moves >= 10 {
            continue;
        }

        let State {
            moves,
            red: cur_red,
            blue: cur_blue,
        } = s;

        for &(dy, dx) in &directions {
            let (new_red, red_in_hole) =
                move_ball(cur_red, (dy, dx), &board, hole);
            let (new_blue, blue_in_hole) =
                move_ball(cur_blue, (dy, dx), &board, hole);

            if blue_in_hole {
                continue;
            }

            if red_in_hole {
                println!("{}", (moves + 1) as i32);
                return;
            }

            let (final_red, final_blue) = if new_red == new_blue {
                adjust_positions(cur_red, cur_blue, new_red, (dy, dx))
            } else {
                (new_red, new_blue)
            };

            if visited.insert((final_red, final_blue)) {
                q.push_back(State {
                    red: final_red,
                    blue: final_blue,
                    moves: s.moves + 1,
                });
            }
        }
    }

    println!("{}", -1);
}

struct State {
    red: (i32, i32),
    blue: (i32, i32),
    moves: usize,
}

fn move_ball(
    pos: (i32, i32),
    dir: (i32, i32),
    board: &[Vec<char>],
    hole: (i32, i32),
) -> ((i32, i32), bool) {
    let (mut y, mut x) = pos;
    let (dy, dx) = dir;

    loop {
        let ny = y + dy;
        let nx = x + dx;

        if board[ny as usize][nx as usize] == '#' {
            break;
        }

        y = ny;
        x = nx;

        if (y, x) == hole {
            return ((y, x), true);
        }
    }

    ((y, x), false)
}

fn adjust_positions(
    old_red: (i32, i32),
    old_blue: (i32, i32),
    new_pos: (i32, i32),
    dir: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let (dy, dx) = dir;

    let red_dist =
        (old_red.0 - new_pos.0).abs() + (old_red.1 - new_pos.1).abs();
    let blue_dist =
        (old_blue.0 - new_pos.0).abs() + (old_blue.1 - new_pos.1).abs();

    if red_dist > blue_dist {
        ((new_pos.0 - dy, new_pos.1 - dx), new_pos)
    } else {
        (new_pos, (new_pos.0 - dy, new_pos.1 - dx))
    }
}

fn get_inputs() -> (usize, usize, Vec<Vec<char>>) {
    let mut buf = String::new();
    match File::open("input.txt") {
        Ok(mut f) => {
            f.read_to_string(&mut buf).unwrap();
        }
        Err(_) => {
            io::stdin()
                .read_to_string(&mut buf)
                .unwrap();
        }
    }
    let mut input = buf.split_whitespace();
    let n = input
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let m = input
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let board = input
        .take(n)
        .map(|line| line.chars().collect())
        .collect();

    (n, m, board)
}
