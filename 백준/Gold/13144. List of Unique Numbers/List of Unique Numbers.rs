use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let nums: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut l = 0;
    let mut r = 0;
    let mut result = 0;
    let mut set = vec![false; 100001];

    while l < n {
        if r < n && !set[nums[r]] {
            set[nums[r]] = true;
            r += 1;
        } else {
            result += r - l;
            set[nums[l]] = false;
            l += 1;
        }
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
