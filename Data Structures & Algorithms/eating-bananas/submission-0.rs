impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = 0;

        for ele in piles.iter() {
            if r < *ele {
                r = *ele;
            }
        }

        while l < r {
            let mid = l + (r - l) / 2;
            if Self::try_solve(&piles, mid, h) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        r
    }

    fn try_solve(piles: &Vec<i32>, k: i32, h: i32) -> bool {
        let mut count = 0;
        for ele in piles.iter() {
            count += (ele / k);
            if ele % k != 0 {
                count += 1;
            }
            if count > h {
                return false;
            }
        }

        true
    }
}