use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let class: Vec<Vec<_>> = (0..n)
        .map(|_| (0..m).map(|_| next!(&mut tokens, usize)).collect())
        .collect();

    let mut students: Vec<(usize, usize)> = Vec::new();
    for c in 0..n {
        for s in 0..m {
            students.push((class[c][s], c));
        }
    }
    students.sort_by_key(|&(s, _)| s);

    let mut l = 0;
    let mut result = usize::MAX;
    let mut count = vec![0; n + 1];
    let mut selected_class_cnt = 0;

    for r in 0..students.len() {
        let (rs, rc) = students[r];
        if count[rc] == 0 {
            selected_class_cnt += 1;
        }

        count[rc] += 1;

        while selected_class_cnt == n {
            let (ls, lc) = students[l];
            result = result.min(rs - ls);

            if count[lc] == 1 {
                selected_class_cnt -= 1;
            }
            count[lc] -= 1;
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
