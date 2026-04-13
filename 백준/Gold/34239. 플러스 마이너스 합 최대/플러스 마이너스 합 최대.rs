use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr = (0..n).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();

    let inf = 1_i64 << 60;
    let mut p = 0i64;
    let mut me = 0i64;
    let mut mo = inf;
    let mut ans = -inf;

    for i in 1..=n {
        let x = arr[i - 1];
        if i % 2 == 1 {
            p += x;
        } else {
            p -= x;
        }

        ans = ans.max(p - me);
        if mo < inf {
            ans = ans.max(-p - mo);
        }

        if i % 2 == 0 {
            me = me.min(p);
        } else {
            mo = mo.min(-p);
        }
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
