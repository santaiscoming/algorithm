impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut cnt = [0; 128];
        for b in s.bytes() {
            cnt[b as usize] += 1;
        }

        let mut chars = cnt
                .into_iter()
                .enumerate()
                .filter(|&(_, cnt)| cnt > 0)
                .map(|(idx, cnt)| (idx as u8 as char, cnt))
                .collect::<Vec<_>>();
        chars.sort_by(|a, b| b.1.cmp(&a.1));

        let mut result = String::new();
        for (c, count) in chars {
            for _ in 0..count {
                result.push(c);
            }
        }

        result
    }
}