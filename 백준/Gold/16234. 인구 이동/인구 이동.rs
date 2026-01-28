use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, Read},
    vec,
};

const DIRECTION: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];

fn main() {
    let mut tokens = read!();
    let (n, l, r) = next!(&mut tokens, usize, usize, usize);
    let mut mat = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect::<Vec<_>>();

    let mut days = 0;

    loop {
        let mut visited = vec![vec![false; n]; n];
        let mut unions = Vec::new();

        (0..n)
            .flat_map(|x| {
                (0..n)
                    .map(move |y| (x, y))
                    .collect::<Vec<_>>()
            })
            .for_each(|(x, y)| {
                if !visited[y][x] {
                    let union =
                        bfs(&mut visited, &mut mat, x, y, n as i32, l, r);

                    if union.len() >= 2 {
                        unions.push(union);
                    }
                }
            });

        if unions.is_empty() {
            break;
        }

        days += 1;
        unions.iter().for_each(|union| {
            let size = union.len();
            let acc = union
                .iter()
                .map(|&(x, y)| mat[y][x])
                .sum::<usize>()
                / size;
            union
                .iter()
                .for_each(|&(x, y)| mat[y][x] = acc);
        });
    }

    println!("{days}");
}

fn bfs(
    visited: &mut Vec<Vec<bool>>,
    mat: &mut Vec<Vec<usize>>,
    sx: usize,
    sy: usize,
    n: i32,
    l: usize,
    r: usize,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::from([(sx, sy)]);

    let mut q = VecDeque::from([(sx, sy)]);
    visited[sy][sx] = true;
    while let Some(v) = q.pop_front() {
        let (cx, cy) = v;

        for (dx, dy) in DIRECTION {
            let (nx, ny) = (cx as i32 + dx, cy as i32 + dy);
            if [nx >= 0, nx < n, ny >= 0, ny < n]
                .iter()
                .all(|&v| v)
            {
                let (nx, ny) = (nx as usize, ny as usize);
                let diff = mat[ny][nx].abs_diff(mat[cy][cx]);

                if [diff >= l, diff <= r, !visited[ny][nx]]
                    .iter()
                    .all(|&v| v)
                {
                    visited[ny][nx] = true;
                    q.push_back((nx, ny));
                    result.insert((nx, ny));
                }
            }
        }
    }

    result
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
