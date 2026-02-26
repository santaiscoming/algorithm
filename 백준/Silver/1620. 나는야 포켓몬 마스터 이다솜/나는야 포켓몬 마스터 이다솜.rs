use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let names = (0..n)
        .map(|_| next!(&mut tokens, String))
        .collect::<Vec<_>>();
    let targets = (0..m)
        .map(|_| next!(&mut tokens, String))
        .collect::<Vec<_>>();

    let mut table = HashMap::<String, String>::new();
    for (id, name) in names.iter().enumerate() {
        let id = (id + 1).to_string();

        if !table.contains_key(name) {
            table.insert(name.clone(), id.clone());
            table.insert(id, name.clone());
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for target in &targets {
        writeln!(out, "{}", table.get(target).unwrap()).unwrap();
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
