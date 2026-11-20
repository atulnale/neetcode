impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if k == 0 {
            return false;
        }
        let mut set: HashSet<i32> = HashSet::new();
        let mut j = 0usize;
        while j < k as usize && j < nums.len() {
            if set.contains(&nums[j]) {
                return true;
            }
            set.insert(nums[j]);
            j += 1;
        }
        if j < k as usize {
            return false;
        }
        while j < nums.len() {
            if set.contains(&nums[j]) {
                return true;
            }
            set.remove(&nums[j - k as usize]);
            set.insert(nums[j]);
            j += 1;
        }

        false
    }
}