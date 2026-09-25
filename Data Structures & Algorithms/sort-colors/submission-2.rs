impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut left: usize = 0;
        let mut right = nums.len() - 1;
        let mut curr: i32 = 0;
        while curr <= right as i32 {
            if nums[curr as usize] == 0 {
                let temp = nums[left];
                nums[left] = nums[curr as usize];
                nums[curr as usize] = temp;
                left += 1;
            } else if nums[curr as usize] == 2 {
                let temp = nums[right];
                nums[right] = nums[curr as usize];
                nums[curr as usize] = temp;
                curr -= 1;
                right -= 1;
            }
            curr += 1;
        }
    }
}
