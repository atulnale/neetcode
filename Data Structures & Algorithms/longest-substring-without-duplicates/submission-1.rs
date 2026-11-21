impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l: i32 = -1;
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut ans = 0;
        for (pos, ch) in s.chars().enumerate() {
            l = map
                .get(&ch)
                .map_or(l, |&val| if val as i32 > l { val as i32 } else { l });

            ans = ans.max(pos as i32 - l);

            map.insert(ch, pos);
        }

        ans as i32
    }
}