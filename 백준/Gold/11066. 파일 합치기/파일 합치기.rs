use std::{
    fs::File,
    io::{self, Read},
    usize,
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);
    (0..t).for_each(|_| {
        let k = next!(&mut tokens, usize);
        let arr: Vec<_> = (0..k).map(|_| next!(&mut tokens, i32)).collect();

        let mut sum = vec![0; k + 1];
        for i in 0..k {
            sum[i + 1] = sum[i] + arr[i];
        }

        let mut dp = vec![vec![-1; k]; k];
        let res = recur(0, k - 1, &sum, &mut dp);

        println!("{}", res);
    });
}

fn recur(l: usize, r: usize, sum: &[i32], dp: &mut Vec<Vec<i32>>) -> i32 {
    if l >= r {
        return 0;
    }

    if dp[l][r] != -1 {
        return dp[l][r];
    }

    let mut ret = i32::MAX;

    let merge = sum[r + 1] - sum[l];

    for i in l..r {
        let left = recur(l, i, sum, dp);
        let right = recur(i + 1, r, sum, dp);

        ret = ret.min(left + right + merge);
    }
    dp[l][r] = ret;

    ret
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
