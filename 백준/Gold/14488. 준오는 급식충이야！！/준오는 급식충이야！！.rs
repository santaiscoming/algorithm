use std::{
    cmp::Reverse,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, t) = next!(&mut tokens, usize, f64);
    let pos: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();
    let speed: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();
    let t = (t * 10000.0) as i128;

    let mut events = Vec::new();
    for i in 0..n {
        let p = (pos[i] * 10000) as i128;
        let s = speed[i] as i128;
        events.push((p - s * t, 1));
        events.push((p + s * t, -1));
    }
    events.sort_by_key(|&(p, s)| (p, Reverse(s)));

    let mut cnt = 0;
    for (_, typ) in events {
        if typ == 1 {
            cnt += 1;
        } else {
            cnt -= 1;
        }

        if cnt == n {
            println!("{}", 1);
            return;
        }
    }

    println!("{}", 0);
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
