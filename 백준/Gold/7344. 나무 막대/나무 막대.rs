use std::{
    fs::File,
    io::{self, Read},
    vec,
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);
    for _ in 0..t {
        let n = next!(&mut tokens, usize);
        let mut arr: Vec<_> =
            (0..n).map(|_| next!(&mut tokens, usize, usize)).collect();

        let res = solve(n, &mut arr);
        println!("{}", res);
    }
}

fn solve(n: usize, arr: &mut Vec<(usize, usize)>) -> usize {
    if n == 1 {
        return 1;
    }

    arr.sort();

    let mut visited = vec![false; n];
    let mut ret = 0;

    for i in 0..n {
        if visited[i] {
            continue;
        }

        ret += 1;
        let mut cw = arr[i].1;
        visited[i] = true;

        for j in i + 1..n {
            if !visited[j] && arr[j].1 >= cw {
                visited[j] = true;
                cw = arr[j].1;
            }
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
