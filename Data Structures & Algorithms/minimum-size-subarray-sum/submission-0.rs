impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r = 0;
        let mut ans = i32::MAX;
        let mut curr_sum = nums[r];
        let mut found = false;
        while r < nums.len() {
            if curr_sum >= target {
                ans = ans.min(1 + r as i32 - l as i32);
                curr_sum = curr_sum.saturating_sub(nums[l]);
                found = true;
                l += 1;
            } else {
                r += 1;
                if r < nums.len() {
                    curr_sum += nums[r];
                }
            }
        }
        return if found { ans } else { 0 };
    }
}