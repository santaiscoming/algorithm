use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let mut arr: Vec<_> = (0..n).map(|_| next!(&mut tokens, usize)).collect();

    arr.sort();
    let mut result = usize::MAX;

    for i in 0..n {
        for j in i + 3..n {
            let elder = arr[i] + arr[j];
            let mut l = i + 1;
            let mut r = j - 1;
            
            while l < r {
                let younger = arr[l] + arr[r];
                
                result = result.min(elder.abs_diff(younger));
                if younger < elder {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
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
