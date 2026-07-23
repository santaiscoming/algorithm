impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let l = flowerbed.len();

        for i in 0..l {
            if flowerbed[i] == 0 {
                let prev = i == 0 || flowerbed[i - 1] == 0;
                let next = i == l - 1 || flowerbed[i + 1] == 0;

                if prev && next {
                    flowerbed[i] = 1;
                    n -= 1;

                    if n <= 0 {
                        return true;
                    }
                }
            }
        }

        n <= 0
    }
}