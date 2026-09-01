impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
let mut ans = vec![0; 2 * nums.len()];

        for (pos, &i) in nums.iter().enumerate() {
            ans[pos] = nums[pos];
            ans[nums.len() + pos] = nums[pos];
        }
        ans
    }
}
