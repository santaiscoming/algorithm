use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, i64);
    let mut nums: Vec<_> = (0..n).map(|_| next!(&mut tokens, i64)).collect();

    let mut min_diff = i64::MAX;
    nums.sort();
    for (i, &cur) in nums.iter().enumerate() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (right + left) / 2;
            let diff = nums[mid] - cur;
            if diff < m {
                left = mid + 1;
            } else if diff >= m {
                if min_diff > diff {
                    min_diff = diff;
                }

                right = mid - 1;
            }
        }
    }

    println!("{}", min_diff);
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
