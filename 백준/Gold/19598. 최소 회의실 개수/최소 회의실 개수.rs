use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr: Vec<_> =
        (0..n).map(|_| next!(&mut tokens, usize, usize)).collect();

    let mut events = Vec::new();
    for (s, e) in arr {
        events.push((s, 1));
        events.push((e, -1));
    }
    events.sort();

    let mut result = 0;
    let mut curr = 0;

    for (_, e) in events {
        curr += e;
        result = result.max(curr);
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
