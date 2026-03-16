use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let axis: Vec<_> = (0..n).map(|_| next!(&mut tokens, i64, i64)).collect();

    let mut events: Vec<(i64, i64)> = Vec::new();
    for (x, y) in axis {
        events.push((x, 1));
        events.push((y, -1));
    }
    events.sort();

    let mut cnt = 0i64;
    let mut prev = events[0].0;
    let mut result = 0i64;

    for &(pos, typ) in &events {
        if cnt > 0 {
            result += pos - prev;
        }
        cnt += typ;
        prev = pos;
    }

    println!("{}", result);
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