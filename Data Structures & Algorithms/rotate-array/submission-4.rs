impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        if k == 0 {
            return;
        }
        Self::reverse(nums, 0, nums.len() - 1);
        Self::reverse(nums, 0, k as usize-1);
        Self::reverse(nums, k as usize, nums.len() - 1);
    }
    fn reverse(nums: &mut Vec<i32>, mut left: usize, mut right: usize) {
        while left < right {
            (nums[left], nums[right]) = (nums[right], nums[left]);
            left += 1;
            right -= 1;
        }
    }
}