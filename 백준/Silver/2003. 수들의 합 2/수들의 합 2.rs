use std::{
    fs::File,
    io::{self, Read},
    usize, vec,
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let nums: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut prefix = vec![0; n + 1];
    for i in 1..n + 1 {
        prefix[i] = prefix[i - 1] + nums[i - 1]
    }

    let mut result = 0;
    for s in 0..=n {
        let mut l = s + 1;
        let mut r = n;

        while l <= r {
            let mid = (l + r) / 2;
            let target = prefix[mid] - prefix[s];

            if target == m {
                result += 1;
                break;
            } else if target > m {
                r = mid - 1
            } else {
                l = mid + 1
            }
        }
    }
    println!("{}", result)
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
