use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, Read},
    result, usize, vec,
};

const DIRECTIONS: [(i32, i32); 8] =
    [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

fn main() {
    let mut tokens = read!();
    let (n, m, k) = next!(&mut tokens, usize, usize, usize);
    let mut A: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let mut trees = (0..m)
        .map(|_| next!(&mut tokens, usize, usize, usize))
        .collect::<Vec<_>>();

    let mut tree_map =
        vec![vec![VecDeque::<(usize, usize, usize)>::new(); n]; n];
    trees.sort_by_key(|&(_, _, age)| age);
    for (y, x, age) in trees {
        let (x, y) = (x - 1, y - 1);
        tree_map[y][x].push_back((age, x, y));
    }
    let mut nutri_map = vec![vec![5; n]; n];

    for _ in 0..k {
        // 봄, 여름
        for y in 0..n {
            for x in 0..n {
                let mut death = 0;
                let mut temp = VecDeque::new();
                while let Some(v) = tree_map[y][x].pop_front() {
                    let (age, _, _) = v;

                    if nutri_map[y][x] >= age {
                        nutri_map[y][x] -= age;
                        temp.push_back((age + 1, x, y));
                    } else {
                        death += age / 2;
                    }
                }
                tree_map[y][x] = temp;
                nutri_map[y][x] += death;
            }
        }

        // 가을
        let mut temp = Vec::new();
        for y in 0..n {
            for x in 0..n {
                for (age, _, _) in &tree_map[y][x] {
                    if age % 5 == 0 {
                        for (dx, dy) in DIRECTIONS {
                            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                            let n = n as i32;

                            if nx >= 0 && nx < n && ny >= 0 && ny < n {
                                let (nx, ny) = (nx as usize, ny as usize);
                                temp.push((nx, ny));
                            }
                        }
                    }
                }
            }
        }
        for (x, y) in temp {
            tree_map[y][x].push_front((1, x, y));
        }

        // 겨울
        for y in 0..n {
            for x in 0..n {
                nutri_map[y][x] += A[y][x]
            }
        }
    }
    let result: usize = tree_map
        .iter()
        .flat_map(|r| r.iter().map(|t| t.len()))
        .sum();
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
