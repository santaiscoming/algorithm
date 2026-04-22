use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (Lx, Ly, n) = next!(&mut tokens, usize, usize, usize);
    let query = (0..n)
        .map(|_| next!(&mut tokens, usize, usize, usize, usize, usize))
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut boxs = Vec::with_capacity(n);

    for (lx, ly, lz, px, py) in query {
        let xs = px;
        let xe = px + lx;
        let ys = py;
        let ye = py + ly;

        let mut h = 0;

        for &(sx, ex, sy, ey, top) in &boxs {
            if xs < ex && sx < xe && ys < ey && sy < ye {
                if h < top {
                    h = top;
                }
            }
        }

        let top = h + lz;
        ans = ans.max(top);
        boxs.push((xs, xe, ys, ye, top));
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
