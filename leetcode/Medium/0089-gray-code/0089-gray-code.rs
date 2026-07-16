impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let count = 1 << n;
        (0..count).map(|i| i ^ (i >> 1)).collect()
    }
}