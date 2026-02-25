use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);
    let tcs = (0..t)
        .map(|_| {
            let n = next!(&mut tokens, usize);
            let scores = (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect::<Vec<usize>>();
            (n, scores)
        })
        .collect::<Vec<_>>();

    for (_, scores) in tcs {
        let counter = scores
            .iter()
            .fold(HashMap::new(), |mut acc, &team| {
                *acc.entry(team).or_insert(0) += 1;
                acc
            });

        let mut teams: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut rank = 1;
        for &team in &scores {
            if counter[&team] == 6 {
                teams
                    .entry(team)
                    .or_insert(Vec::new())
                    .push(rank);
                rank += 1;
            }
        }

        let mut result: Vec<(usize, usize, usize)> = Vec::new();
        for (&team, ranks) in &teams {
            let top4: usize = ranks[..4].iter().sum();
            let fifth = ranks[4];
            result.push((team, top4, fifth));
        }

        result.sort_by_key(|&(_, score, fifth)| (score, fifth));
        println!("{}", result[0].0);
    }
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
    ($tokens:expr) => {
        $tokens.next().unwrap()
    };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
