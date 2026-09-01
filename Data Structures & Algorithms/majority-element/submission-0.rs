impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut counter = 1;

        for num in nums.iter().skip(1) {
            if *num == max {
                counter += 1;
            } else {
                counter -= 1;
                if counter < 0 {
                    max = *num;
                    counter = 1;
                }
            }
        }
        max
    }
}
