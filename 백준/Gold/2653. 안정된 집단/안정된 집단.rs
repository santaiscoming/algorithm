use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufWriter, Read, Write},
};

fn main() {
    let mut tokens = read!();
    let n = next!(&mut tokens, usize);

    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = next!(&mut tokens, i32);
        }
    }

    let mut parent: Vec<usize> = (0..n).collect();

    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 0 {
                union(i, j, &mut parent);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            let root_i = find(i, &mut parent);
            let root_j = find(j, &mut parent);

            if root_i == root_j && grid[i][j] != 0 {
                println!("0");
                return;
            }
            if root_i != root_j && grid[i][j] != 1 {
                println!("0");
                return;
            }
        }
    }

    let mut group_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..n {
        let root = find(i, &mut parent);
        group_map.entry(root).or_default().push(i);
    }

    for members in group_map.values() {
        if members.len() < 2 {
            println!("0");
            return;
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    writeln!(out, "{}", group_map.len()).unwrap();
    for members in group_map.values() {
        for (idx, &member) in members.iter().enumerate() {
            if idx > 0 {
                write!(out, " ").unwrap();
            }
            write!(out, "{}", member + 1).unwrap();
        }
        writeln!(out).unwrap();
    }
}

fn find(i: usize, parent: &mut [usize]) -> usize {
    if parent[i] == i {
        i
    } else {
        parent[i] = find(parent[i], parent);
        parent[i]
    }
}

fn union(i: usize, j: usize, parent: &mut [usize]) {
    let root_i = find(i, parent);
    let root_j = find(j, parent);
    if root_i != root_j {
        parent[root_j] = root_i;
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
