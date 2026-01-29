use std::{
    collections::HashSet,
    ffi::NulError,
    fs::File,
    io::{self, Read},
    ops::{Index, IndexMut},
    vec,
};

type Cube = Vec<Vec<Vec<char>>>;

#[derive(Clone, Copy)]
enum C {
    U,
    D,
    F,
    B,
    L,
    R,
}
impl Index<C> for Cube {
    type Output = Vec<Vec<char>>;
    fn index(&self, c: C) -> &Self::Output {
        &self[c as usize]
    }
}
impl IndexMut<C> for Cube {
    fn index_mut(&mut self, c: C) -> &mut Self::Output {
        &mut self[c as usize]
    }
}
impl From<char> for C {
    fn from(value: char) -> Self {
        match value {
            'U' => C::U,
            'D' => C::D,
            'F' => C::F,
            'B' => C::B,
            'L' => C::L,
            'R' => C::R,
            _ => panic!(""),
        }
    }
}

fn main() {
    let mut tokens = read!();
    let t = next!(&mut tokens, usize);
    let tcs = (0..t)
        .map(|_| {
            let n = next!(&mut tokens, usize);
            let lines = (0..n)
                .map(|_| {
                    next!(&mut tokens, String)
                        .trim()
                        .to_string()
                })
                .collect::<Vec<_>>();
            (n, lines)
        })
        .collect::<Vec<_>>();

    for (_, lines) in tcs {
        let mut cube: Cube = vec![
            vec![vec!['w'; 3]; 3], // U
            vec![vec!['y'; 3]; 3], // D
            vec![vec!['r'; 3]; 3], // F
            vec![vec!['o'; 3]; 3], // B
            vec![vec!['g'; 3]; 3], // L
            vec![vec!['b'; 3]; 3], // R
        ];

        for command in lines {
            let [side, turn] = command
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let times = if turn == '+' {
                1
            } else {
                3
            };

            for _ in 0..times {
                rotate(&mut cube, side);
            }
        }

        for row in &cube[C::U] {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn rotate(cube: &mut Cube, side: char) {
    let side = C::from(side);
    cube[side] = rotate_right(&cube[side]);

    match side {
        C::U => {
            let temp = cube[C::F][0].clone();
            cube[C::F][0] = cube[C::R][0].clone();
            cube[C::R][0] = cube[C::B][0].clone();
            cube[C::B][0] = cube[C::L][0].clone();
            cube[C::L][0] = temp;
        }
        C::D => {
            let temp = cube[C::F][2].clone();
            cube[C::F][2] = cube[C::L][2].clone();
            cube[C::L][2] = cube[C::B][2].clone();
            cube[C::B][2] = cube[C::R][2].clone();
            cube[C::R][2] = temp;
        }
        C::F => {
            let temp: Vec<char> = cube[C::U][2].clone();
            for i in 0..3 {
                cube[C::U][2][i] = cube[C::L][2 - i][2];
            }
            for i in 0..3 {
                cube[C::L][i][2] = cube[C::D][0][i];
            }
            for i in 0..3 {
                cube[C::D][0][i] = cube[C::R][2 - i][0];
            }
            for i in 0..3 {
                cube[C::R][i][0] = temp[i];
            }
        }
        C::B => {
            let temp: Vec<char> = cube[C::U][0].clone();
            for i in 0..3 {
                cube[C::U][0][i] = cube[C::R][i][2];
            }
            for i in 0..3 {
                cube[C::R][i][2] = cube[C::D][2][2 - i];
            }
            for i in 0..3 {
                cube[C::D][2][i] = cube[C::L][i][0];
            }
            for i in 0..3 {
                cube[C::L][i][0] = temp[2 - i];
            }
        }
        C::L => {
            let temp: Vec<char> = (0..3)
                .map(|i| cube[C::U][i][0])
                .collect();
            for i in 0..3 {
                cube[C::U][i][0] = cube[C::B][2 - i][2];
            }
            for i in 0..3 {
                cube[C::B][i][2] = cube[C::D][2 - i][0];
            }
            for i in 0..3 {
                cube[C::D][i][0] = cube[C::F][i][0];
            }
            for i in 0..3 {
                cube[C::F][i][0] = temp[i];
            }
        }
        C::R => {
            let temp: Vec<char> = (0..3)
                .map(|i| cube[C::U][i][2])
                .collect();
            for i in 0..3 {
                cube[C::U][i][2] = cube[C::F][i][2];
            }
            for i in 0..3 {
                cube[C::F][i][2] = cube[C::D][i][2];
            }
            for i in 0..3 {
                cube[C::D][i][2] = cube[C::B][2 - i][0];
            }
            for i in 0..3 {
                cube[C::B][i][0] = temp[2 - i];
            }
        }
    }
}

fn rotate_right(old: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = transpose(&old);
    result
        .iter_mut()
        .for_each(|v| v.reverse());

    result
}

fn rotate_left(old: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = transpose(&old);
    result.reverse();
    result
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let n = matrix.len();

    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| matrix[c][r].clone())
                .collect()
        })
        .collect()
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
