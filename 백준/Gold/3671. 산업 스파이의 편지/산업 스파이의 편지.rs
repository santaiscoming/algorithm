use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let c = next!(&mut tokens, u64);
    let tcs = (0..c)
        .map(|_| next!(&mut tokens, String))
        .collect::<Vec<String>>();

    for s in tcs {
        let n = s.len();
        let mut candi = HashSet::new();
        let mut visited = vec![false; n];

        permu(
            &mut String::new(),
            0,
            &s.chars().collect::<Vec<_>>(),
            &mut visited,
            &mut candi,
        );

        let mut res = 0;
        for v in candi {
            if is_prime(v) {
                res += 1
            }
        }

        println!("{}", res);
    }
}

fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    }

    let bound = (num as f64).sqrt() as usize;
    for i in 2..=bound {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}

fn permu(
    acc: &mut String,
    depth: usize,
    arr: &Vec<char>,
    visited: &mut Vec<bool>,
    set: &mut HashSet<usize>,
) {
    let n = arr.len();
    if depth == n {
        return;
    }

    for i in 0..n {
        if !visited[i] {
            acc.push(arr[i]);
            visited[i] = true;
            set.insert(acc.to_string().parse::<usize>().unwrap());
            permu(acc, depth + 1, arr, visited, set);
            visited[i] = false;
            acc.pop();
        }
    }
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
