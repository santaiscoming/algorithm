use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| (next!(&mut tokens, i32) + 1) as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cnt = vec![0; 3];
    recur(0, 0, n, &mut cnt, &grid);
    for v in cnt {
        println!("{}", v);
    }
}

fn recur(
    r: usize,
    c: usize,
    size: usize,
    cnt: &mut Vec<usize>,
    grid: &Vec<Vec<usize>>,
) {
    if size == 1 {
        cnt[grid[r][c]] += 1;
        return;
    }

    let mut all_same = true;
    let first = grid[r][c];
    'outer: for nr in r..r + size {
        for nc in c..c + size {
            if first != grid[nr][nc] {
                all_same = false;
                break 'outer;
            }
        }
    }

    if all_same {
        cnt[first] += 1;
        return;
    }

    let divide = size / 3;
    for i in 0..3 {
        for j in 0..3 {
            recur(r + i * divide, c + j * divide, divide, cnt, grid);
        }
    }
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
