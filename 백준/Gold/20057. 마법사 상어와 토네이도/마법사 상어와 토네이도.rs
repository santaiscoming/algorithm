use std::{
    fs::File,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
type Grid = Vec<Vec<usize>>;

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, i32);

    let mut grid: Grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, usize))
                .collect()
        })
        .collect();

    let mut cr = n / 2;
    let mut cc = n / 2;
    let limit = 2 * n - 1;
    let mut move_cnt = 1;

    let mut result = 0;
    let mut j = 0;

    'a: for i in 0..limit {
        let times = if i != (limit - 1) {
            2
        } else {
            move_cnt -= 1;
            1
        };

        for _ in 0..times {
            for _ in 0..move_cnt {
                let d = j % 4;
                let (dr, dc) = DIRECTIONS[d as usize];
                grid[cr as usize][cc as usize] = 0;
                cr += dr;
                cc += dc;
                if cr < 0 || cc < 0 {
                    break 'a;
                }

                result +=
                    move_sand(cr as usize, cc as usize, d as usize, &mut grid);
            }
            j += 1;
        }

        move_cnt += 1;
    }

    println!("{}", result);
}

fn move_sand(yr: usize, yc: usize, d: usize, grid: &mut Grid) -> usize {
    let mut sand_map = vec![vec![0; 5]; 5];
    let origin = grid[yr][yc];
    grid[yr][yc] = 0;
    let one = origin * 1 / 100;
    let two = origin * 2 / 100;
    let five = origin * 5 / 100;
    let seven = origin * 7 / 100;
    let ten = origin * 10 / 100;
    let a: usize = origin
        - five
        - [one, two, seven, ten]
            .iter()
            .sum::<usize>()
            * 2;

    sand_map[1][3] = one;
    sand_map[3][3] = one;

    sand_map[0][2] = two;
    sand_map[4][2] = two;

    sand_map[2][0] = five;

    sand_map[1][2] = seven;
    sand_map[3][2] = seven;

    sand_map[1][1] = ten;
    sand_map[3][1] = ten;

    sand_map[2][1] = a;

    for _ in 0..d {
        sand_map = rotate(&sand_map)
    }

    let mut rest = 0;

    for r in 0..5i32 {
        for c in 0..5i32 {
            if sand_map[r as usize][c as usize] == 0 {
                continue;
            }

            let n = grid.len();
            let mr: i32 = r - 2;
            let mc: i32 = c - 2;
            let nr = yr.wrapping_add(mr as usize);
            let nc = yc.wrapping_add(mc as usize);
            let sand = sand_map[r as usize][c as usize];

            if nr >= n || nc >= n {
                rest += sand;
            } else {
                grid[nr][nc] += sand
            }
        }
    }

    rest
}

fn rotate(grid: &Grid) -> Grid {
    let mut new_grid = transpose(grid);
    new_grid.reverse();

    new_grid
}

fn transpose(grid: &Grid) -> Grid {
    let mut result = vec![vec![0; 5]; 5];

    for r in 0..5 {
        for c in 0..5 {
            result[c][r] = grid[r][c]
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
    ($tokens:expr) => {
        $tokens.next().unwrap()
    };
    ($tokens:expr, $($t:ty),+) => {
        ($($tokens.next().unwrap().parse::<$t>().unwrap()),+)
    };
}
