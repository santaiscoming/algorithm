use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m, h) = next!(&mut tokens, usize, usize, usize);
    let info = (0..m)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    let mut ladder = vec![vec![false; n + 1]; h + 1];
    for &(a, b) in &info {
        ladder[a][b] = true;
    }

    for cnt in 0..=3 {
        if dfs(&mut ladder, n, h, cnt, 1, 1) {
            println!("{}", cnt);
            return;
        }
    }
    println!("-1");
}

fn check(ladder: &Vec<Vec<bool>>, n: usize, h: usize) -> bool {
    for sy in 1..=n {
        let mut py = sy;
        for x in 1..=h {
            if py < n && ladder[x][py] {
                py += 1;
            } else if py > 1 && ladder[x][py - 1] {
                py -= 1;
            }
        }

        if py != sy {
            return false;
        }
    }
    true
}

fn dfs(
    ladder: &mut Vec<Vec<bool>>,
    n: usize,
    h: usize,
    cnt: usize,
    sy: usize,
    sx: usize,
) -> bool {
    if cnt == 0 {
        return check(ladder, n, h);
    }

    for y in sy..=h {
        let k = if y == sy {
            sx
        } else {
            1
        };

        for x in k..n {
            if [ladder[y][x], ladder[y][x - 1], ladder[y][x + 1]]
                .iter()
                .all(|&x| !x)
            {
                ladder[y][x] = true;
                if dfs(ladder, n, h, cnt - 1, y, x + 2) {
                    return true;
                }
                ladder[y][x] = false;
            }
        }
    }
    false
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
    ($tokens:expr) => { $tokens.next().unwrap() };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
