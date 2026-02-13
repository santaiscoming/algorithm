use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);

    let mut belt = (0..2 * n)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<usize>>();

    let mut robot = vec![false; 2 * n];
    let mut result = 1;

    loop {
        // 1. 회전
        belt.rotate_right(1);
        robot.rotate_right(1);

        if robot[n - 1] {
            robot[n - 1] = false;
        }

        for i in (0..2 * n - 1).rev() {
            if robot[i] && !robot[i + 1] && belt[i + 1] >= 1 {
                robot[i] = false;
                robot[i + 1] = true;
                belt[i + 1] -= 1;
            }

            if robot[n - 1] {
                robot[n - 1] = false;
            }
        }

        if !robot[0] && belt[0] >= 1 {
            robot[0] = true;
            belt[0] -= 1;
        }

        //
        if is_done(&belt, k) {
            break;
        }
        result += 1
    }

    println!("{}", result)
}

fn is_done(belt: &Vec<usize>, k: usize) -> bool {
    belt.iter().filter(|&&v| v == 0).count() >= k
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
