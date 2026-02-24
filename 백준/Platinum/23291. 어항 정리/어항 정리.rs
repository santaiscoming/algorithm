use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    let mut tokens = read!();
    let (n, k) = next!(&mut tokens, usize, usize);
    let mut fishes = (0..n)
        .map(|_| next!(&mut tokens, i64))
        .collect::<Vec<i64>>();

    let mut result = 0;

    loop {
        let max = *fishes.iter().max().unwrap();
        let min = *fishes.iter().min().unwrap();

        if max - min <= k as i64 {
            println!("{}", result);
            return;
        }

        result += 1;

        // 1. 최소 물고기 어항에 +1
        let min = *fishes.iter().min().unwrap();
        fishes
            .iter_mut()
            .filter(|v| **v == min)
            .for_each(|v| *v += 1);

        // 2. 어항 쌓기 (시계 90도 회전 반복)
        let mut grid: Vec<Vec<i64>> = fishes
            .iter()
            .map(|&v| vec![v])
            .collect();

        loop {
            let block_w = grid
                .iter()
                .take_while(|col| col.len() > 1)
                .count();
            if block_w == 0 {
                if grid.len() < 2 {
                    break;
                }
                let a = grid.remove(0);
                grid[0].extend(a);
            } else {
                let block_h = grid[0].len();
                let remain = grid.len() - block_w;

                if block_h > remain {
                    break;
                }

                let block: Vec<Vec<i64>> = grid.drain(0..block_w).collect();
                let mut rotated = vec![vec![0i64; block_w]; block_h];
                for c in 0..block_w {
                    for r in 0..block_h {
                        rotated[r][block_w - 1 - c] = block[c][r];
                    }
                }

                for i in 0..block_h {
                    grid[i].extend(rotated[i].iter());
                }
            }
        }

        // 3. 물고기 수 조절
        adjust(&mut grid);

        // 4. 일렬로 펴기
        fishes = flatten(&grid);

        // 5. 반 접기 (2번 반복)
        grid = fishes
            .iter()
            .map(|&v| vec![v])
            .collect();
        for _ in 0..2 {
            let half = grid.len() / 2;
            let left: Vec<Vec<i64>> = grid.drain(0..half).collect();

            for i in 0..half {
                let src = &left[half - 1 - i];
                let reversed: Vec<i64> = src.iter().rev().copied().collect();
                grid[i].extend(reversed);
            }
        }

        // 6. 물고기 수 조절
        adjust(&mut grid);

        // 7. 일렬로 펴기
        fishes = flatten(&grid);
    }
}

fn adjust(grid: &mut Vec<Vec<i64>>) {
    let col = grid.len();
    let mut diff = vec![vec![0i64; 0]; col];
    for c in 0..col {
        diff[c] = vec![0i64; grid[c].len()];
    }

    for c in 0..col {
        for r in 0..grid[c].len() {
            if c + 1 < col && r < grid[c + 1].len() {
                let d = (grid[c][r] - grid[c + 1][r]).abs() / 5;
                if d > 0 {
                    if grid[c][r] > grid[c + 1][r] {
                        diff[c][r] -= d;
                        diff[c + 1][r] += d;
                    } else {
                        diff[c][r] += d;
                        diff[c + 1][r] -= d;
                    }
                }
            }

            if r + 1 < grid[c].len() {
                let d = (grid[c][r] - grid[c][r + 1]).abs() / 5;
                if d > 0 {
                    if grid[c][r] > grid[c][r + 1] {
                        diff[c][r] -= d;
                        diff[c][r + 1] += d;
                    } else {
                        diff[c][r] += d;
                        diff[c][r + 1] -= d;
                    }
                }
            }
        }
    }

    for c in 0..col {
        for r in 0..grid[c].len() {
            grid[c][r] += diff[c][r];
        }
    }
}

fn flatten(grid: &Vec<Vec<i64>>) -> Vec<i64> {
    let mut result = Vec::new();
    for col in grid {
        for &v in col {
            result.push(v);
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
