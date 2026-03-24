use std::{
    fs::File,
    io::{self, Read},
};

const MOD: u64 = 1_000_000;

fn main() {
    let mut tokens = read!();
    let mut n = next!(&mut tokens, u64);

    let mul = |m1: [[u64; 2]; 2], m2: [[u64; 2]; 2]| -> [[u64; 2]; 2] {
        let mut tmp = [[0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    tmp[i][j] = (tmp[i][j] + m1[i][k] * m2[k][j]) % MOD;
                }
            }
        }
        tmp
    };

    let mut result = [[1, 0], [0, 1]];
    let mut base = [[1, 1], [1, 0]];

    while n > 0 {
        if n % 2 == 1 {
            result = mul(result, base);
        }

        base = mul(base, base);
        n /= 2;
    }

    println!("{}", result[0][1]);
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
