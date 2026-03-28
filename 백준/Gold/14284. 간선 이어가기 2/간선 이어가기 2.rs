use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let edge = (0..m)
        .map(|_| next!(&mut tokens, usize, usize, usize))
        .collect::<Vec<_>>();
    let (s, t) = next!(&mut tokens, usize, usize);

    let mut adj = vec![vec![]; n + 1];
    for (u, v, w) in edge {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut dist = vec![usize::MAX; n + 1];
    let mut q = BinaryHeap::new();
    dist[s] = 0;
    q.push((Reverse(0), s));

    while let Some((Reverse(d), u)) = q.pop() {
        if d <= dist[u] {
            for &(v, w) in &adj[u] {
                let next = d + w;
                if next < dist[v] {
                    dist[v] = next;
                    q.push((Reverse(next), v));
                }
            }
        }
    }

    println!("{}", dist[t]);
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
