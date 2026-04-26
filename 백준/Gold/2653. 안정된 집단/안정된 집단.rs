use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut tokens = read!();
    // 입력이 비어있을 수 있는 예외 처리 (백준 채점 환경 대비)
    let n = next!(&mut tokens, usize);

    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = next!(&mut tokens, i32);
        }
    }

    let mut vis = vec![false; n];
    let mut groups = Vec::new();

    for i in 0..n {
        if vis[i] {
            continue;
        }

        let mut cur_grp = Vec::new();

        for j in 0..n {
            if grid[i][j] == 0 {
                cur_grp.push(j);
            }
        }

        if cur_grp.len() < 2 {
            println!("0");
            return;
        }

        for &member in &cur_grp {
            if grid[i] != grid[member] {
                println!("0");
                return;
            }
            vis[member] = true;
        }

        groups.push(cur_grp);
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "{}", groups.len()).unwrap();
    for grp in groups {
        for (idx, &member) in grp.iter().enumerate() {
            if idx > 0 {
                write!(out, " ").unwrap();
            }

            write!(out, "{}", member + 1).unwrap();
        }
        writeln!(out).unwrap();
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
