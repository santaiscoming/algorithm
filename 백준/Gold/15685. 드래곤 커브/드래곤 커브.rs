use std::{
    collections::HashSet,
    fs::File,
    io::{self, Read},
    vec,
};

const DIRS: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);
    let lines = (0..n)
        .map(|_| next!(&mut tokens, i32, i32, usize, usize))
        .collect::<Vec<_>>();

    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for (x, y, d, g) in lines {
        set.insert((x, y));

        let mut curve: Vec<usize> = vec![d];
        (0..g).for_each(|_| {
            let next = &curve
                .iter()
                .rev()
                .map(|&v| (v + 1) % 4)
                .collect::<Vec<_>>();
            curve.extend(next);
        });
        curve
            .iter()
            .fold((x, y), |(cx, cy), &to| {
                let (dx, dy) = DIRS[to];
                let next = (cx + dx, cy + dy);
                set.insert(next);

                next
            });
    }

    let mut result = 0;
    (0..100).for_each(|y| {
        (0..100).for_each(|x| {
            if [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)]
                .iter()
                .all(|&(x, y)| set.contains(&(x, y)))
            {
                result += 1;
            }
        });
    });
    println!("{result}");
}

#[macro_export]
macro_rules! read {
    () => {{
        let mut buf = String::new();
        match File::open("input.txt") {
            Ok(mut f) => f.read_to_string(&mut buf).unwrap(),
            Err(_) => io::stdin()
                .read_to_string(&mut buf)
                .unwrap(),
        };
        Box::leak(buf.into_boxed_str()).split_ascii_whitespace()
    }};
}

#[macro_export]
macro_rules! next {
    ($tokens:expr) => { $tokens.next().unwrap() };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
