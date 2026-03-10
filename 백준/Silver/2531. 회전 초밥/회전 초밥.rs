use std::{
    fs::File,
    io::{self, Read},
    vec,
};

fn main() {
    let mut tokens = read!();
    let (n, d, k, c) = next!(&mut tokens, usize, usize, usize, usize);
    let sushi: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    let mut result = 0;
    let mut state = 0;
    let mut count = vec![0; d + 1];
    for i in 0..k - 1 {
        let v = sushi[i];
        if count[v] == 0 {
            state += 1
        }

        count[v] += 1;
    }
    result += state;

    for l in 0..n {
        let l_v = sushi[l];

        // 오른쪾 늘리기
        let r = (l + k - 1) % n;
        let r_v = sushi[r];
        if count[r_v] == 0 {
            state += 1;
        }
        count[r_v] += 1;

        // 쿠폰
        let bonus = if count[c] == 0 { 1 } else { 0 };
        result = result.max(state + bonus);

        // k개가 되어 갱신

        // 왼쪽 감소
        if count[l_v] == 1 {
            state -= 1;
        }
        count[l_v] -= 1;
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
