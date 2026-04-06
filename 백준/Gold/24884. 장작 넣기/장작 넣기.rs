use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, w, t, k) = next!(&mut tokens, usize, usize, usize, usize);
    let arr: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut arr: Vec<_> = arr.iter().map(|&v| (v, false)).collect();
    let res = recur(t - 1, w, k, &mut arr);
    println!("{}", res);
}

fn recur(t: usize, pos: usize, k: usize, arr: &mut [(usize, bool)]) -> usize {
    let n = arr.len();

    // 1. 화력 감소
    let og: Vec<_> = arr.iter().copied().collect();
    for i in 0..n {
        if og[i].1 {
            arr[i].1 = false;
            continue;
        }

        let mut dec = 0;
        if i > 0 && og[i - 1].0 > 0 {
            dec += 1;
        }
        if i + 1 < n && og[i + 1].0 > 0 {
            dec += 1;
        }
        let dec = 3 - dec;
        arr[i].0 = arr[i].0.saturating_sub(dec);
    }

    if t == 0 {
        return if arr.iter().filter(|&&v| v.0 > 0).count() >= k {
            1
        } else {
            0
        };
    }

    let mut ret = 0;
    for i in -1i32..=1 {
        let nx = pos.wrapping_add(i as usize);
        if nx < n {
            // 2. 장작 넣기
            let mut new_arr: Vec<_> = arr.iter().copied().collect();
            if new_arr[nx].0 > 0 {
                new_arr[nx].1 = true;
            }
            ret += recur(t - 1, nx, k, &mut new_arr);
        }
    }

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
