use std::{
    cmp::max,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);

    let mut par = vec![0; n + 1];
    let mut sum = vec![0; n + 1];
    let mut root = vec![0; n + 1];
    let mut node = 0;

    let mut out = String::new();

    for i in 1..=n {
        let q = next!(&mut tokens);

        if q == "push" {
            let x = next!(&mut tokens, i64);
            node += 1;
            par[node] = root[i - 1];
            sum[node] = sum[par[node]] + x;
            root[i] = node;
        } else if q == "pop" {
            root[i] = par[root[i - 1]];
        } else if q == "restore" {
            let k = next!(&mut tokens, usize);
            root[i] = root[k];
        } else {
            root[i] = root[i - 1];
            out.push_str(&format!("{}\n", sum[root[i]]));
        }
    }

    print!("{}", out);
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
