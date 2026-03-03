use std::{
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, s, e) = next!(&mut tokens, usize, usize, usize);
    let s = s - 1;
    let e = e - 1;

    let mut adj = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u = next!(&mut tokens, usize) - 1;
        let v = next!(&mut tokens, usize) - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    if s == e {
        println!("First");
        return;
    }

    let mut parent = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    visited[s] = true;
    q.push_back(s);

    while let Some(u) = q.pop_front() {
        if u == e {
            break;
        }
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                parent[v] = u;
                q.push_back(v);
            }
        }
    }

    let mut path = vec![];
    let mut cur = e;
    while cur != s {
        path.push(cur);
        cur = parent[cur];
    }
    path.push(s);
    path.reverse();

    let mut second_wins = false;
    for i in (1..path.len() - 1).step_by(2) {
        if adj[path[i]].len() > 2 {
            second_wins = true;
            break;
        }
    }

    if second_wins {
        println!("Second");
    } else {
        println!("First");
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
