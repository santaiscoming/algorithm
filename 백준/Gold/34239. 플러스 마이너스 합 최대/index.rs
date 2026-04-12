use std::{
    fs::File,
    io::{self, Read},
    vec,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr = (0..n).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();

    let mut ret: i64 = i64::MIN;
    let mut acc: i64 = 0;
    let mut min_even: i64 = 0;
    let mut max_odd: i64 = i64::MIN;

    for r in 1..n + 1 {
        let sign = if r % 2 != 0 { 1 } else { -1 };

        acc += arr[r - 1] * sign;
        ret = ret.max(acc - min_even).max(max_odd - acc);

        if r % 2 == 0 {
            min_even = min_even.min(acc);
        } else {
            max_odd = max_odd.max(acc);
        }
    }

    println!("{}", ret);
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
