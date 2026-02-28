use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, i64);
    let nums = (0..n).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();

    let mut counter: HashMap<i64, i64> = HashMap::new();
    counter.insert(0, 1);

    let mut sum: i64 = 0;
    let mut answer: i64 = 0;

    for &num in &nums {
        sum += num;
        if let Some(&cnt) = counter.get(&(sum - k)) {
            answer += cnt;
        }

        counter.entry(sum).and_modify(|v| *v += 1).or_insert(1);
    }

    println!("{}", answer);
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
