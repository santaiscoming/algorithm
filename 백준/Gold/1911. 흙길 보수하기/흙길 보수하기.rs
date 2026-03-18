use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let mut arr = (0..n)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    arr.sort();

    let mut l = arr[0].0;
    let mut r = arr[0].1;
    let mut result = 0;

    for i in 1..n {
        let (ll, rr) = arr[i];
        let nr = if (r - l) % k == 0 { r } else { r + (k - (r - l) % k) };

        if ll <= nr {
            r = rr
        } else {
            let wood =
                if (r - l) % k == 0 { (r - l) / k } else { (r - l) / k + 1 };
            result += wood;
            l = ll;
            r = rr;
        }
    }
    result += if (r - l) % k == 0 { (r - l) / k } else { (r - l) / k + 1 };

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
