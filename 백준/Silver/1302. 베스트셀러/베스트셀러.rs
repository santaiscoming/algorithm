use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let names = (0..n)
        .map(|_| next!(&mut tokens, String))
        .collect::<Vec<_>>();

    let counter: HashMap<String, usize> =
        names.iter().fold(HashMap::new(), |mut acc, cur| {
            acc.entry(cur.clone()).and_modify(|v| *v += 1).or_insert(1);
            acc
        });
    let max = &counter.values().copied().max().unwrap();

    let mut candidates = Vec::new();
    for (k, v) in counter {
        if v == *max {
            candidates.push(k);
        }
    }

    candidates.sort_unstable();
    println!("{}", candidates[0]);
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
