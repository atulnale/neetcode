impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let num = 100007;
        for i in 0..nums.len() {
            if nums[i] < 1 || nums[i] > nums.len() as i32 {
                nums[i] = 0;
            }
        }
        for i in 0..nums.len() {
            let ele = nums[i] % num;
            if ele == 0 {
                continue;
            }
            nums[ele as usize - 1] += num;
        }
        for i in 0..nums.len() {
            if nums[i] < num {
                return i as i32 + 1;
            }
        }
        nums.len() as i32 + 1
    }
}