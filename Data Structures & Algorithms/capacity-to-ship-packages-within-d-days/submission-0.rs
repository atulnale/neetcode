impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut l = i32::MIN;
        let mut r = 0;
        for ele in weights.iter() {
            l = l.max(*ele);
            r += ele;
        }

        while l < r {
            let mid = l + (r - l) / 2;
            if Self::try_fix(&weights, days, mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r
    }

    pub fn try_fix(weights: &Vec<i32>, days: i32, cap: i32) -> bool {
        let mut count = 0;
        let mut curr_sum = 0;
        for ele in weights {
            if curr_sum + ele > cap {
                count += 1;
                if count > days {
                    return false;
                }
                curr_sum = *ele;
            } else {
                curr_sum += ele;
            }
        }
        if curr_sum != 0 {
            count += 1;
        }
        if count > days {
            return false;
        }

        true
    }
}