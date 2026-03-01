use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, i64);
    let n = next!(&mut tokens, usize);
    let a = (0..n).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();
    let m = next!(&mut tokens, usize);
    let b = (0..m).map(|_| next!(&mut tokens, i64)).collect::<Vec<_>>();

    let mut prefix_a: HashMap<i64, i64> = HashMap::new();
    for i in 0..n {
        let mut sum = 0;
        for j in i..n {
            sum += a[j];
            prefix_a.entry(sum).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    let mut result = 0;
    for i in 0..m {
        let mut sum = 0;
        for j in i..m {
            sum += b[j];
            let target = t - sum;

            if let Some(&cnt) = prefix_a.get(&target) {
                result += cnt;
            }
        }
    }

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
