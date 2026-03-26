use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    
    writeln!(out, "{}", (n - 1) * (n - 1)).unwrap();

    for i in 2..=n {
        writeln!(out, "1 {}", i).unwrap();
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
