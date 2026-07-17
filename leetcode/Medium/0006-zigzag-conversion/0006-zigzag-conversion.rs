impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows == 1 || num_rows >= s.len() {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut r = 0;
        let mut is_down = false;

        for c in s.chars() {
            rows[r].push(c);
            if r == 0 || r == num_rows - 1 {
                is_down = !is_down; // 맨 위/맨 아래에서 방향 전환
            }
            if is_down { r += 1 } else { r -= 1 }
        }

        rows.concat()
    }
}