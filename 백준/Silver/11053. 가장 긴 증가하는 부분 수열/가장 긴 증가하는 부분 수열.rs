use std::{
    fs::File,
    io::{self, Read},
};

// bottom-up
// let mut dp = vec![1; n];
// for i in 1..n {
//     let cur = arr[i];
//     for j in (0..=i - 1).rev() {
//         let prev = arr[j];

//         if cur > prev {
//             dp[i] = dp[i].max(dp[j] + 1);
//         }
//     }
// }
// println!("{}", dp.iter().max().unwrap());

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr = (0..n)
        .map(|_| next!(&mut tokens, usize))
        .collect::<Vec<_>>();

    let mut res = 0;
    let mut dp = vec![-1; n];
    for i in 0..n {
        res = res.max(top_down(i, &arr, &mut dp));
    }
    println!("{}", res);
}

fn top_down(i: usize, arr: &Vec<usize>, dp: &mut Vec<i32>) -> i32 {
    if dp[i] != -1 {
        return dp[i];
    }

    let mut ret = 1i32;
    for j in 0..i {
        if arr[i] > arr[j] {
            ret = ret.max(top_down(j, arr, dp) + 1);
        }
    }
    dp[i] = ret;

    dp[i]
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
