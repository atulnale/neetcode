impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if target < nums[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r as i32
    }
}