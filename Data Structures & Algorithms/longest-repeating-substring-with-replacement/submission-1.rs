impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut map: HashMap<u8, usize> = HashMap::new();
        let v = s.as_bytes();
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        while r < v.len() {
            let len = r - l + 1;
            let val = *map.get(&v[r]).unwrap_or(&0);
            map.insert(v[r], val + 1);
            let max = Solution::get_max(&mut map);
            if len - max <= k {
                ans = ans.max(len);
            } else {
                let t = *map.get(&v[l]).unwrap_or(&0);
                map.insert(v[l], t - 1);
                l += 1;
            }
            r += 1;
        }

        ans as i32
    }
    fn get_max(map: &HashMap<u8, usize>) -> usize {
        let mut max = 0usize;
        for (_, count) in map.iter() {
            max = max.max(*count);
        }

        max
    }
}