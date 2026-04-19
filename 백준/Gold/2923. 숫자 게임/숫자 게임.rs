use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let arr = (0..n)
        .map(|_| next!(&mut tokens, usize, usize))
        .collect::<Vec<_>>();

    let mut ans = String::new();
    let mut count_a = [0; 101];
    let mut count_b = [0; 101];

    for (a, b) in arr {
        count_a[a] += 1;
        count_b[b] += 1;

        let mut copy_a = count_a;
        let mut copy_b = count_b;

        let mut sum = 0;
        let mut min_ptr_a = 1;
        let mut max_ptr_b = 100;

        while min_ptr_a <= 100 && max_ptr_b >= 1 {
            while min_ptr_a <= 100 && copy_a[min_ptr_a] == 0 {
                min_ptr_a += 1;
            }
            while max_ptr_b >= 1 && copy_b[max_ptr_b] == 0 {
                max_ptr_b -= 1;
            }

            if min_ptr_a > 100 || max_ptr_b < 1 {
                break;
            }

            let matched = copy_a[min_ptr_a].min(copy_b[max_ptr_b]);

            sum = sum.max(min_ptr_a + max_ptr_b);
            copy_a[min_ptr_a] -= matched;
            copy_b[max_ptr_b] -= matched;
        }

        ans.push_str(&format!("{sum}\n"));
    }

    println!("{}", ans);
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
