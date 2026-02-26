use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);

    let not_listen = (0..n)
        .map(|_| next!(&mut tokens, String))
        .collect::<HashSet<_>>();
    let not_see = (0..m)
        .map(|_| next!(&mut tokens, String))
        .collect::<HashSet<_>>();

    let mut intersec = not_listen.intersection(&not_see).collect::<Vec<_>>();
    intersec.sort_unstable();

    println!("{}", intersec.len());
    intersec.iter().for_each(|v| {
        println!("{}", v);
    });
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
