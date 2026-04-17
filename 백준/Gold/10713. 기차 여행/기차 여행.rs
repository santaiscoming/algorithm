use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let m = next!(&mut tokens, usize);
    let path = (0..m)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<_>>();

    let mut diff = vec![0i64; n + 1];
    for i in 0..m - 1 {
        let l = path[i].min(path[i + 1]);
        let r = path[i].max(path[i + 1]);
        diff[l] += 1;
        diff[r] -= 1;
    }

    let mut cnt = 0i64;
    let mut ans = 0i64;

    for i in 1..n {
        cnt += diff[i];
        let t = next!(&mut tokens, i64);
        let ic = next!(&mut tokens, i64);
        let buy = next!(&mut tokens, i64);

        ans += (cnt * t).min(cnt * ic + buy);
    }

    println!("{}", ans);
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
