use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let mut arr: Vec<i64> = (0..n).map(|_| next!(&mut tokens, i64)).collect();
    let queries: Vec<_> = (0..k)
        .map(|_| next!(&mut tokens, usize, usize, i64))
        .collect();

    arr.sort();
    for (l, r, x) in queries {
        for i in l..=r {
            arr[i - 1] += x;
        }

        arr.sort();
    }

    println!(
        "{}",
        arr.iter()
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
