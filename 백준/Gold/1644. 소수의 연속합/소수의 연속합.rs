use std::{
    fs::File,
    io::{self, Read},
    usize, vec,
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);

    if n <= 1 {
        println!("{}", 0);
        return;
    }

    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<_> = is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| v.then_some(i))
        .collect();
    let mut prefix = vec![0];
    for &p in &primes {
        prefix.push(prefix.last().unwrap() + p);
    }

    let mut result = 0;
    for i in 0..prefix.len() {
        let target = prefix[i] + n;
        let mut left = i + 1;
        let mut right = prefix.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;

            if prefix[mid] == target {
                result += 1;
                break;
            } else if prefix[mid] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
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
