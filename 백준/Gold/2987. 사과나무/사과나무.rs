use std::{
    fs::File,
    i64,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let land = (0..3)
        .map(|_| next!(&mut tokens, i32, i32))
        .collect::<Vec<_>>();
    let n = next!(&mut tokens, usize);
    let apple = (0..n)
        .map(|_| next!(&mut tokens, i32, i32))
        .collect::<Vec<_>>();

    let mut res = 0;
    let size = calc(land[0], land[1], land[2]);
    for c in apple {
        let mut sum = 0f32;
        for a in 0..3 {
            for b in a + 1..3 {
                sum += calc(land[a], land[b], c);
            }
        }

        if sum == size {
            res += 1
        }
    }

    println!("{:.1}", size);
    println!("{}", res);
}

fn calc(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> f32 {
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;

    (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by)).abs() as f32 / 2.0
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
