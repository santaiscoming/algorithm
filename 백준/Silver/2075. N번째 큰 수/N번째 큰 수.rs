use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut q: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut n = 0;

    for (idx, line) in stdin.lock().lines().enumerate() {
        let line = line.unwrap();
        if idx == 0 {
            n = line.trim().parse().unwrap();
            continue;
        }

        for token in line.split_whitespace() {
            let v: i64 = token.parse().unwrap();

            if q.len() < n {
                q.push(Reverse(v));
            } else if v > q.peek().unwrap().0 {
                q.pop();
                q.push(Reverse(v));
            }
        }
    }

    println!("{}", q.peek().unwrap().0);
}
