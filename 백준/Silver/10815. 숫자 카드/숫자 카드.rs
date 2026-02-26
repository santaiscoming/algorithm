use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let cards = (0..n).map(|_| next!(&mut tokens, i32)).collect::<Vec<_>>();
    let m = next!(&mut tokens, usize);
    let targets = (0..m).map(|_| next!(&mut tokens, i32)).collect::<Vec<_>>();

    let mut table = HashMap::new();
    for v in cards {
        if let None = table.get(&v) {
            table.insert(v, v);
        }
    }

    let mut result = Vec::new();
    for v in targets {
        if table.contains_key(&v) {
            result.push(1);
        } else {
            result.push(0);
        }
    }
    println!(
        "{}",
        result
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
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
