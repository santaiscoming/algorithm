use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let nums: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut l = 0;
    let mut r = 0;
    let mut k = k;
    let mut cnt = 0;
    let mut result = 0;
    let mut removed = vec![false; 1_000_001];

    while l < n {
        if r < n && nums[r] % 2 == 1 && k > 0 {
            removed[r] = true;
            k -= 1;
            r += 1;
        } else if r < n && nums[r] % 2 == 0 {
            r += 1;
            cnt += 1;
            result = result.max(cnt);
        } else {
            if removed[l] {
                removed[l] = false;
                k += 1;
            }
            if nums[l] % 2 == 0 {
                cnt -= 1;
            }

            l += 1;
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
