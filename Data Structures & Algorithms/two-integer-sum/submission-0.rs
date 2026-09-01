impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            if let Some(&complement_index) = map.get(&(target - value)) {
                return vec![complement_index as i32, index as i32];
            }
            map.insert(value, index);
        }
        vec![-1, -1]
    }
}
