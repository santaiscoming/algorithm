use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let customers: Vec<_> =
        (0..n).map(|_| next!(&mut tokens, usize, usize)).collect();

    let mut events: Vec<(usize, usize)> = Vec::new();
    for &(s, e) in &customers {
        if s > e {
            events.push((e, s));
        }
    }

    if events.is_empty() {
        println!("{}", m);
        return;
    }

    events.sort();

    let mut total = 0;
    let mut pl = events[0].0;
    let mut pr = events[0].1;
    for &(l, r) in &events[1..] {
        if l <= pr {
            pr = pr.max(r);
        } else {
            total += pr - pl;
            pl = l;
            pr = r;
        }
    }
    total += pr - pl;

    let result = m + total * 2;
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
