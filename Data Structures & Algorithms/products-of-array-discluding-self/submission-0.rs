impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut pref = vec![1; nums.len() + 1];
        let mut post = vec![1; nums.len() + 1];

        for i in 0..nums.len() {
            pref[i + 1] = pref[i] * nums[i];
        }
        for i in (0..nums.len()).rev() {
            post[i] = post[i + 1] * nums[i];
        }
        let mut res = vec![1; nums.len()];
        for i in 0..res.len() {
            res[i] = pref[i] * post[i + 1];
        }
        res
    }
}