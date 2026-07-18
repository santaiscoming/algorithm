impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev = 0i32;
        
        while x != 0 {
            let d = x % 10;
            x /= 10;
            
            if let Some(rev_rev) = rev.checked_mul(10) {
                if let Some(rev_rev_rev) = rev_rev.checked_add(d) {
                    rev = rev_rev_rev;
                } else {
                    return 0;
                }
            } else {
                return 0;
            }
        }
        
        rev
    }
}