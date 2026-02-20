use std::{
    fs::File,
    io::{self, Read},
};

const ICE_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const ROTATION: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

type Grid = Vec<Vec<usize>>;

fn main() {
    let mut tokens = read!();
    let (n, m) = next!(&mut tokens, usize, usize);
    let mut grid: Grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();
    let magics: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let (d, s) = next!(&mut tokens, usize, usize);
            (d - 1, s)
        })
        .collect();

    let mut result = 0;
    for (d, s) in magics {
        // 1. 얼음 낙하
        let mut cr = n as i32 / 2;
        let mut cc = n as i32 / 2;

        for _ in 0..s {
            let (dr, dc) = ICE_DIRECTIONS[d];
            cr += dr;
            cc += dc;

            grid[cr as usize][cc as usize] = 0;
        }

        // 2. 구슬 재배열
        let mut len = 1;
        let mut d = 0;
        let mut cr = n as i32 / 2;
        let mut cc = n as i32 / 2;
        let mut beads = Vec::new();

        'a: loop {
            for _ in 0..2 {
                for _ in 0..len {
                    let (dr, dc) = ROTATION[d];
                    cr += dr;
                    cc += dc;

                    let bead = grid[cr as usize][cc as usize];
                    if bead != 0 {
                        beads.push(bead);
                    }

                    if cr == 0 && cc == 0 {
                        break 'a;
                    }
                }
                d = (d + 1) % 4;
            }

            len += 1;
        }

        // 3. 폭발 후 재정렬
        while let Some(Info {
            one,
            two,
            three,
            new_beads,
        }) = explosion(&beads)
        {
            result += 1 * one + 2 * two + 3 * three;
            beads = new_beads;
        }

        // 4. 그룹 나누기
        let mut new_beads = Vec::new();
        let mut i = 0;
        while i < beads.len() {
            let v = beads[i];
            let mut cnt = 1;
            i += 1;

            while i < beads.len() && beads[i] == v {
                cnt += 1;
                i += 1;
            }
            new_beads.push(cnt);
            new_beads.push(v);
        }
        beads = new_beads;
        let cap = n * n - 1;
        if beads.len() > cap {
            beads.truncate(cap);
        }

        // 5. grid 다시 그리기
        let mut len = 1;
        let mut d = 0;
        let mut cr = n as i32 / 2;
        let mut cc = n as i32 / 2;
        let mut new_grid = vec![vec![0; n]; n];
        let mut i = 0;

        'a: while i < beads.len() {
            for _ in 0..2 {
                for _ in 0..len {
                    let (dr, dc) = ROTATION[d];
                    cr += dr;
                    cc += dc;

                    if cr == 0 && cc == 0 {
                        break 'a;
                    }
                    if i == beads.len() {
                        break 'a;
                    }

                    new_grid[cr as usize][cc as usize] = beads[i];
                    i += 1;
                }
                d = (d + 1) % 4;
            }
            len += 1;
        }
        grid = new_grid;
    }
    println!("{}", result);
}

struct Info {
    new_beads: Vec<usize>,
    one: usize,
    two: usize,
    three: usize,
}
fn explosion(beads: &Vec<usize>) -> Option<Info> {
    let mut info = Info {
        new_beads: Vec::new(),
        one: 0,
        two: 0,
        three: 0,
    };

    let mut stack = Vec::new();
    let mut cnt = 0;
    for (i, &v) in beads.iter().enumerate() {
        if stack.is_empty() {
            stack.push(v);
            cnt += 1;
            continue;
        }

        if *stack.last().unwrap() == v {
            cnt += 1;
        } else {
            if cnt >= 4 {
                for _ in 0..cnt {
                    let v = stack.pop().unwrap();
                    match v {
                        1 => info.one += 1,
                        2 => info.two += 1,
                        3 => info.three += 1,
                        _ => unreachable!(),
                    }
                }
            }
            cnt = 1;
        }
        stack.push(v);

        if i == beads.len() - 1 {
            if cnt >= 4 {
                for _ in 0..cnt {
                    let v = stack.pop().unwrap();
                    match v {
                        1 => info.one += 1,
                        2 => info.two += 1,
                        3 => info.three += 1,
                        _ => unreachable!(),
                    }
                }
            }
        }
    }

    if [info.one, info.two, info.three]
        .iter()
        .all(|&v| v == 0)
    {
        return None;
    }

    info.new_beads = stack;
    Some(info)
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