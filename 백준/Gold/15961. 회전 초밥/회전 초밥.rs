use std::{
    fs::File,
    io::{self, Read},
    vec,
};

fn main() {
    let mut tokens = read!();
    let (n, d, k, c) = next!(&mut tokens, usize, usize, usize, usize);
    let sushi: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut res = 0;
    let mut state = 0;
    let mut count = vec![0; 3001];

    for i in 0..k {
        let cur = sushi[i];
        if count[cur] == 0 {
            state += 1;
            res = res.max(state);
        }
        count[cur] += 1;
    }
    if count[c] == 0 {
        res = res.max(state + 1);
    }

    for i in k..n + k - 1 {
        let prev = sushi[(i - k) % n];
        let curr = sushi[i % n];

        if count[curr] == 0 {
            state += 1;
        }
        count[curr] += 1;

        if count[prev] == 1 {
            state -= 1;
        }
        count[prev] -= 1;

        let bouns = if count[c] == 0 { 1 } else { 0 };
        res = res.max(state + bouns);
    }

    println!("{}", res)
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
