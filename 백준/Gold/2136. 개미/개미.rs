use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, l) = next!(&mut tokens, usize, i64);
    let arr = (0..n)
        .map(|i| (next!(&mut tokens, i64), i + 1))
        .collect::<Vec<_>>();

    let mut l_cnt = 0;
    let mut tl = 0;
    let mut tr = 0;
    for i in 0..n {
        let (t, _) = arr[i];

        if t.is_negative() {
            tl = tl.max(t.abs());
            l_cnt += 1;
        } else {
            tr = tr.max(l - t.abs());
        }
    }

    let mut pos = arr
        .iter()
        .map(|&(v, i)| (v.abs() as usize, i))
        .collect::<Vec<_>>();
    pos.sort();

    if tl > tr {
        println!("{} {}", pos[l_cnt - 1].1, tl)
    } else {
        println!("{} {}", pos[l_cnt].1, tr)
    }
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
