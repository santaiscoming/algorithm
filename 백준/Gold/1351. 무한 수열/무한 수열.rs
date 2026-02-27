use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    mem,
};

fn main() {
    let mut tokens = read!();
    let (n, p, q) = next!(&mut tokens, usize, usize, usize);

    let mut memo = HashMap::new();
    memo.entry(0).or_insert(1);

    let result = dp(&mut memo, n, p, q);
    println!("{}", result);
}

fn dp(memo: &mut HashMap<usize, usize>, n: usize, p: usize, q: usize) -> usize {
    if n <= 0 {
        return memo.get(&0).copied().unwrap();
    }
    if let Some(&v) = memo.get(&n) {
        return v;
    }

    let left = dp(memo, n / p, p, q);
    let right = dp(memo, n / q, p, q);
    memo.insert(n, left + right);

    return left + right;
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
