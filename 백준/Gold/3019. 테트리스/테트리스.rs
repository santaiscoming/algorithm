use std::{
    fs::File,
    io::{self, Read},
};

const SHAPES: &[&[&[i32]]] = &[
    &[&[0, 0, 0, 0], &[0]],                      //  I
    &[&[0, 0]],                                  //  O
    &[&[0, 0, 1], &[1, 0]],                      //  S
    &[&[1, 0, 0], &[0, 1]],                      //  Z
    &[&[0, 0, 0], &[0, 1], &[1, 0, 1], &[1, 0]], //  T
    &[&[0, 0], &[0, 1, 1], &[2, 0], &[0, 0, 0]], //  L
    &[&[0, 0], &[0, 0, 0], &[0, 2], &[1, 1, 0]], //  J
];

fn main() {
    let mut tokens = read!();
    let (c, p) = next!(&mut tokens, usize, usize);
    let heights: Vec<i32> = (0..c).map(|_| next!(&mut tokens, i32)).collect();

    let block = SHAPES[p - 1];
    let mut ans = 0;

    for shape in block {
        let w = shape.len();
        if c < w {
            continue;
        }

        for start in 0..=(c - w) {
            let base = heights[start] - shape[0];
            let mut ok = true;

            for i in 1..w {
                if heights[start + i] - shape[i] != base {
                    ok = false;
                    break;
                }
            }

            if ok {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
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
