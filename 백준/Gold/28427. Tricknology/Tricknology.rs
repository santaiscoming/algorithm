use std::{
    fs::File,
    io::{self, Read},
};

const UPPER: usize = 500_001;
const SIEVE_UPPER: usize = 1_000_003;

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let queries = (0..n)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    let mut is_prime = vec![true; SIEVE_UPPER];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i < SIEVE_UPPER {
        if is_prime[i] {
            let mut j = i * i;
            while j < SIEVE_UPPER {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let mut prefix = vec![0usize; UPPER + 1];
    for x in 0..UPPER {
        prefix[x + 1] = prefix[x] + if is_prime[2 * x + 1] { 1 } else { 0 };
    }

    let mut out = String::new();
    for (l, r) in queries {
        let count = prefix[r] - prefix[l];
        out.push_str(&count.to_string());
        out.push('\n');
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
