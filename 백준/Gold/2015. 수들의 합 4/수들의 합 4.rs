use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, i64);
    let nums = (0..n).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();

    let mut result = 0;
    let mut acc = 0;
    let mut counter: HashMap<i64, i64> = HashMap::new();
    counter.insert(0, 1);

    for num in nums {
        acc += num;

        if let Some(v) = counter.get(&(acc - k)) {
            result += v
        }

        counter.entry(acc).and_modify(|v| *v += 1).or_insert(1);
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
