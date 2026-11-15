impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for ele in nums.iter() {
            set.insert(*ele);
        }
        let mut map = HashMap::new();
        for ele in nums {
            Solution::helper(&set, &mut map, ele)
        }
        let mut ans = 0;
        for (k, v) in map.iter() {
            ans = ans.max(*v)
        }
        ans
    }

    fn helper(set: &HashSet<i32>, map: &mut HashMap<i32, i32>, ele: i32) {
        if map.contains_key(&ele) {
            return;
        }
        match map.get(&(ele - 1)) {
            Some(x) => {
                map.insert(ele, x + 1);
            }
            None => {
                if set.contains(&(ele - 1)) {
                    Solution::helper(set, map, ele - 1);
                    Solution::helper(set, map, ele);
                } else {
                    map.insert(ele, 1);
                }
            }
        }
    }
}