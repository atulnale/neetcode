impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let sa: Vec<char> = s.chars().collect();
        let mut ansl = 0;
        let mut ansr = 0;
        let mut ans = usize::MAX;

        let mut found = 0;
        let original: HashMap<char, usize> = Self::create_original_map(&t);
        let mut curr: HashMap<char, usize> = HashMap::new();
        let mut required = original.len();
        let mut l = 0usize;
        let mut r = 0usize;
        while r < s.len() {
            match original.get(&sa[r]) {
                Some(&val) => {
                    let count = *curr.get(&sa[r]).unwrap_or(&0usize) + 1;
                    if count == val {
                        found += 1;
                    }
                    curr.insert(sa[r], count);
                }
                None => {}
            }
            while found == required {
                if r + 1 - l < ans {
                    ans = r + 1 - l;
                    ansl = l;
                    ansr = r;
                }

                match original.get(&sa[l]) {
                    Some(&val) => {
                        let count = *curr.get(&sa[l]).unwrap_or(&1usize);
                        if count == val {
                            found -= 1;
                        }
                        curr.insert(sa[l], count - 1);
                    }
                    None => {}
                }
                l += 1;
            }
            r += 1;
        }
        if ans == usize::MAX {
            return "".to_string();
        }
        s[ansl..=ansr].to_string()
    }
    fn create_original_map(t: &str) -> HashMap<char, usize> {
        let mut map = HashMap::new();
        for ch in t.chars() {
            let count = *map.get(&ch).unwrap_or(&0usize);
            map.insert(ch, count + 1);
        }
        map
    }
}