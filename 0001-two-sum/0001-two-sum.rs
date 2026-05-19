impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();
        let mut res = Vec::new();

        'outer: for i in 0..n {
            for j in 0..n {
                if i != j {
                    let a = nums[i];
                    let b = nums[j];

                    if a + b == target {
                        res.push(i as i32);
                        res.push(j as i32);
                        break 'outer;
                    }
                }
            }
        }

        res
    }
}