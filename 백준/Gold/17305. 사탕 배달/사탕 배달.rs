use std::{
    cmp::{Reverse, max},
    fs::File,
    io::{self, Read},
    iter::Rev,
    vec,
};

fn main() {
    let mut tokens = read!();
    let (n, w) = next!(&mut tokens, usize, usize);
    let mut arr = (0..n)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    let mut three = Vec::new();
    let mut five = Vec::new();
    for (w, s) in arr {
        if w == 3 {
            three.push(s);
        } else {
            five.push(s);
        }
    }
    three.sort_by_key(|&v| Reverse(v));
    five.sort_by_key(|&v| Reverse(v));
    let mut prefix_3 = vec![0; three.len() + 1];
    let mut acc = 0;
    for i in 1..=three.len() {
        acc += three[i - 1];
        prefix_3[i] = acc;
    }
    let mut prefix_5 = vec![0; five.len() + 1];
    acc = 0;
    for i in 1..=five.len() {
        acc += five[i - 1];
        prefix_5[i] = acc;
    }

    let mut ans = 0;
    for i in 0..prefix_3.len() {
        if 3 * i > w {
            break;
        }

        let m3 = prefix_3[i];
        let rest = ((w - 3 * i) / 5).min(five.len());

        ans = ans.max(m3 + prefix_5[rest]);
    }
    println!("{}", ans)
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
