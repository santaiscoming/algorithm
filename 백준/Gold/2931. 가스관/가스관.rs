use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let mut tokens = read!();
    let (r, c) = next!(&mut tokens, usize, usize);
    let mut grid = (0..r)
        .map(|_| next!(&mut tokens, String).chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..r {
        for c in 0..c {
            if grid[r][c] == 'M' {
                start = (r, c)
            }
            if grid[r][c] == 'Z' {
                end = (r, c)
            }
        }
    }

    let mut d = 0;
    let (mut cr, mut cc) = start;
    for dir in 0..4 {
        let (dr, dc) = DIRECTIONS[dir];
        let nr = cr.wrapping_add(dr as usize);
        let nc = cc.wrapping_add(dc as usize);
        if nr >= r || nc >= c {
            continue;
        }
        let ch = grid[nr][nc];
        if ch != '.' && connects(ch)[(dir + 2) % 4] {
            d = dir;
            break;
        }
    }

    loop {
        let (dr, dc) = DIRECTIONS[d];
        let nr = cr.wrapping_add(dr as usize);
        let nc = cc.wrapping_add(dc as usize);

        let next = grid[nr][nc];

        if next == '.' {
            let blocks = ['-', '|', '+', '1', '2', '3', '4'];
            for b in blocks {
                if !is_fit(nr, nc, b, &grid, r, c) {
                    continue;
                }

                let mut nd = d;
                change(&mut nd, b);
                grid[nr][nc] = b;

                if is_possible(nr, nc, nd, &grid, end, r, c) {
                    println!("{} {} {b}", nr + 1, nc + 1);
                    return;
                }
            }
            return;
        }

        change(&mut d, next);
        cr = nr;
        cc = nc;
    }
}

fn is_possible(
    mut cr: usize,
    mut cc: usize,
    mut d: usize,
    grid: &Vec<Vec<char>>,
    end: (usize, usize),
    rows: usize,
    cols: usize,
) -> bool {
    loop {
        let (dr, dc) = DIRECTIONS[d];
        let nr = cr.wrapping_add(dr as usize);
        let nc = cc.wrapping_add(dc as usize);
        if nr >= rows || nc >= cols {
            return false;
        }

        let next = grid[nr][nc];
        if next == '.' {
            return false;
        }

        if (nr, nc) == end {
            return true;
        }

        change(&mut d, next);
        cr = nr;
        cc = nc;
    }
}

fn change(d: &mut usize, next: char) {
    if next == '|' || next == '-' || next == '+' {
        return;
    }

    match (*d, next) {
        (0, '1') => *d = 1,
        (0, '4') => *d = 3,
        (1, '3') => *d = 0,
        (1, '4') => *d = 2,
        (2, '2') => *d = 1,
        (2, '3') => *d = 3,
        (3, '1') => *d = 2,
        (3, '2') => *d = 0,
        _ => {}
    }
}

fn connects(ch: char) -> [bool; 4] {
    match ch {
        '|' => [true, false, true, false],
        '-' => [false, true, false, true],
        '+' => [true, true, true, true],
        '1' => [false, true, true, false],
        '2' => [true, true, false, false],
        '3' => [true, false, false, true],
        '4' => [false, false, true, true],
        _ => [false, false, false, false],
    }
}

fn is_fit(
    row: usize,
    col: usize,
    b: char,
    grid: &Vec<Vec<char>>,
    rows: usize,
    cols: usize,
) -> bool {
    let bc = connects(b);
    for dir in 0..4 {
        let (dr, dc) = DIRECTIONS[dir];
        let nr = row.wrapping_add(dr as usize);
        let nc = col.wrapping_add(dc as usize);
        if nr >= rows || nc >= cols {
            if bc[dir] {
                return false;
            }
            continue;
        }
        let neighbor = grid[nr][nc];
        if neighbor == '.' {
            continue;
        }
        let opp = (dir + 2) % 4;
        let neighbor_connects_here = connects(neighbor)[opp];

        if bc[dir] != neighbor_connects_here {
            return false;
        }
    }
    true
}

#[macro_export]
macro_rules! read {
    () => {{
        let mut buf = String::new();
        match File::open("input.txt") {
            Ok(mut f) => f.read_to_string(&mut buf).unwrap(),
            Err(_) => io::stdin().read_to_string(&mut buf).unwrap(),
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
