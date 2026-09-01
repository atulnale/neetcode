impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut front = 0;
        while front < nums.len() {
            if nums[front] != val {
                nums[left] = nums[front];
                left += 1;
            }
            front += 1;
        }
        left as i32
    }
}
