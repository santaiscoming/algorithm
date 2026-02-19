use std::{
    cmp::Reverse,
    collections::VecDeque,
    fs::File,
    io::{self, Read},
};

const EMPTY: i32 = -2;
const RAINBOW: i32 = 0;
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Grid = Vec<Vec<i32>>;

fn main() {
    let mut tokens = read!();
    let (n, _m) = next!(&mut tokens, usize, usize);
    let mut grid: Grid = (0..n)
        .map(|_| {
            (0..n)
                .map(|_| next!(&mut tokens, i32))
                .collect()
        })
        .collect();

    let mut score = 0;

    loop {
        let mut normal_block_visited = vec![vec![false; n]; n];
        let mut groups = Vec::new();

        for r in 0..n {
            for c in 0..n {
                if grid[r][c] > 0 && !normal_block_visited[r][c] {
                    match find_group_bfs(
                        r,
                        c,
                        &mut grid,
                        &mut normal_block_visited,
                    ) {
                        None => continue,
                        Some(v) => {
                            groups.push(v);
                        }
                    }
                }
            }
        }

        if groups.len() < 1 {
            break;
        }

        groups.sort_by_key(|&(total, a, b, c, _, _)| {
            (Reverse(total), Reverse(a), Reverse(b), Reverse(c))
        });
        let (_, _, _, _, rainbows, blocks) = &groups[0];
        score += remove_blocks(&rainbows, &blocks, &mut grid);

        gravity(&mut grid);
        rotate(&mut grid);
        gravity(&mut grid);
    }
    println!("{}", score);
}

fn find_group_bfs(
    sr: usize,
    sc: usize,
    grid: &Grid,
    normal_block_visited: &mut Vec<Vec<bool>>,
) -> Option<(
    usize,
    usize,
    usize,
    usize,
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
)> {
    let n = grid.len();
    let ref_block = grid[sr][sc];
    let mut visited = vec![vec![false; n]; n];
    let mut q = VecDeque::new();
    let mut rainbows = Vec::new();
    let mut blocks = Vec::new();

    normal_block_visited[sr][sc] = true;
    visited[sr][sc] = true;
    q.push_back((sr, sc));
    blocks.push((sr, sc));

    while let Some((cr, cc)) = q.pop_front() {
        for (dr, dc) in DIRECTIONS {
            let nr = cr.wrapping_add(dr as usize);
            let nc = cc.wrapping_add(dc as usize);

            if nr < n
                && nc < n
                && grid[nr][nc] >= 0
                && (grid[nr][nc] == ref_block || grid[nr][nc] == RAINBOW)
                && !visited[nr][nc]
            {
                if grid[nr][nc] == RAINBOW {
                    rainbows.push((nr, nc));
                } else {
                    blocks.push((nr, nc));
                    normal_block_visited[nr][nc] = true
                }

                visited[nr][nc] = true;
                q.push_back((nr, nc));
            }
        }
    }

    if blocks.len() < 1 || (blocks.len() + rainbows.len()) < 2 {
        return None;
    }

    blocks.sort();
    let (ref_r, ref_c) = blocks[0];

    Some((
        rainbows.len() + blocks.len(),
        rainbows.len(),
        ref_r,
        ref_c,
        rainbows,
        blocks,
    ))
}

fn remove_blocks(
    rainbows: &Vec<(usize, usize)>,
    blocks: &Vec<(usize, usize)>,
    grid: &mut Grid,
) -> usize {
    for &(r, c) in rainbows {
        grid[r][c] = EMPTY;
    }
    for &(r, c) in blocks {
        grid[r][c] = EMPTY;
    }

    let result = rainbows.len() + blocks.len();
    result * result
}

fn gravity(grid: &mut Grid) {
    let n = grid.len();

    for c in 0..n {
        for r in (0..n - 1).rev() {
            let mut sr = r;
            while sr < n - 1
                && grid[sr][c] >= RAINBOW
                && grid[sr + 1][c] == EMPTY
            {
                grid[sr + 1][c] = grid[sr][c];
                grid[sr][c] = EMPTY;
                sr += 1
            }
        }
    }
}

fn rotate(grid: &mut Grid) {
    let n = grid.len();
    let mut new_grid = vec![vec![0i32; n]; n];

    for r in 0..n {
        for c in 0..n {
            new_grid[r][c] = grid[c][r];
        }
    }
    new_grid.reverse();

    *grid = new_grid;
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
