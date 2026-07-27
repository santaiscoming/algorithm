use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = is_water.len();
        let n = is_water[0].len();
        
        let mut height = vec![vec![-1; n]; m];
        let mut q = VecDeque::new();

        for r in 0..m {
            for c in 0..n {
                if is_water[r][c] == 1 {
                    height[r][c] = 0;
                    q.push_back((r, c));
                }
            }
        }

        let dirs: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in dirs {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);

                if nr < m && nc < n && height[nr][nc] == -1 {
                    height[nr][nc] = height[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }

        height
    }
}