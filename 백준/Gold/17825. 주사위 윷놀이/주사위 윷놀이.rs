use std::{
    fs::File,
    io::{self, Read},
};

const END: usize = 21;
const SCORE: [usize; 33] = [
    0, //
    2, 4, 6, 8, 10, 12, 14, 16, 18, 20, //
    22, 24, 26, 28, 30, 32, 34, 36, 38, //
    40, //
    0,  //
    13, 16, 19, //
    22, 24, //
    28, 27, 26, //
    25, //
    30, 35, //
];
const NEXT: [usize; 33] = [
    1, //
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, //
    12, 13, 14, 15, 16, 17, 18, 19, 20, //
    21, //
    21, //
    23, 24, 30, //
    26, 30, //
    28, 29, 30, //
    31, //
    32, 20, //
];
const BLUE_NEXT: [usize; 33] = {
    let mut arr = NEXT;
    arr[5] = 22;
    arr[10] = 25;
    arr[15] = 27;
    arr
};

fn main() {
    let mut tokens = read!();
    let dice: Vec<usize> = (0..10)
        .map(|_| next!(&mut tokens, usize))
        .collect();

    let mut horses = [0usize; 4];
    let result = dfs(0, 0, &dice, &mut horses);
    println!("{result}");
}

fn dfs(
    turn: usize,
    acc_score: usize,
    dice: &Vec<usize>,
    horses: &mut [usize; 4],
) -> usize {
    if turn == 10 {
        return acc_score;
    }

    let mut max_score = 0;

    for i in 0..4 {
        if horses[i] == END {
            continue;
        }

        let mut dest = horses[i];
        for step in 0..dice[turn] {
            if dest == END {
                break;
            }

            if step == 0 {
                dest = BLUE_NEXT[dest]
            } else {
                dest = NEXT[dest]
            }
        }
        if dest != END && horses.contains(&dest) {
            continue;
        }

        let prev = horses[i];
        horses[i] = dest;
        max_score =
            max_score.max(dfs(turn + 1, acc_score + SCORE[dest], dice, horses));
        horses[i] = prev;
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
