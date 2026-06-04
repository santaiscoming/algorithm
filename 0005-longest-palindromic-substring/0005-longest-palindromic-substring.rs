impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut max_len = 0;

        for i in 0..bytes.len() {
            let l1 = Self::expand(bytes, i as i32, i as i32);
            let l2 = Self::expand(bytes, i as i32, i as i32 + 1);
            
            let len = l1.max(l2);

            if len > max_len {
                max_len = len;
                start = i - (len - 1) / 2;
            }
        }

        s[start..start + max_len].to_string()
    }

    fn expand(bytes: &[u8], mut left: i32, mut right: i32) -> usize {
        while left >= 0 && right < bytes.len() as i32 && bytes[left as usize] == bytes[right as usize] {
            left -= 1;
            right += 1;
        }
        (right - left - 1) as usize
    }
}