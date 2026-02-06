use std::{
    fs::File,
    io::{self, Read},
    usize,
};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let board: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let mut horses: Vec<(usize, usize, usize)> = (0..k)
        .map(|_| {
            let (r, c, d) = next!(&mut tokens, usize, usize, usize);
            (r - 1, c - 1, d - 1)
        })
        .collect();

    let mut chess = vec![vec![Vec::new(); n]; n];
    for (i, &horse) in horses.iter().enumerate() {
        let (r, c, _) = horse;
        chess[r][c].push(i);

        if chess[r][c].len() >= 4 {
            println!("0");
            return;
        }
    }

    let mut turn = 0;

    'a: loop {
        turn += 1;

        if turn > 1000 {
            println!("-1");
            return;
        }

        for id in 0..k {
            let (y, x, mut d) = horses[id];
            let (dx, dy) = DIRECTIONS[d];

            let mut nx = x.wrapping_add(dx as usize);
            let mut ny = y.wrapping_add(dy as usize);

            let is_blue = nx >= n || ny >= n || board[ny][nx] == 2;
            if is_blue {
                d ^= 1;
                let (dx, dy) = DIRECTIONS[d];
                nx = x.wrapping_add(dx as usize);
                ny = y.wrapping_add(dy as usize);

                if nx >= n || ny >= n || board[ny][nx] == 2 {
                    horses[id].2 = d;
                    continue;
                }
            }

            let block = board[ny][nx];
            let idx = chess[y][x]
                .iter()
                .position(|&v| v == id)
                .unwrap();
            let rest: Vec<_> = chess[y][x].splice(idx.., []).collect();

            if block == 1 {
                for &h_id in rest.iter().rev() {
                    horses[h_id] = (
                        ny,
                        nx,
                        if h_id == id {
                            d
                        } else {
                            horses[h_id].2
                        },
                    );
                    chess[ny][nx].push(h_id);
                }
            } else {
                for &h_id in &rest {
                    horses[h_id] = (
                        ny,
                        nx,
                        if h_id == id {
                            d
                        } else {
                            horses[h_id].2
                        },
                    );
                }
                chess[ny][nx].extend(rest);
            }

            if chess[ny][nx].len() >= 4 {
                break 'a;
            }
        }
    }

    println!("{}", turn);
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
