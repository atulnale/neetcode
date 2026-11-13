impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut res = String::new();
        for str in strs {
            res.push_str(&format!("{:08b}", str.len() & 0xFF));
            res.push_str(&str);
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        if s.is_empty() {
            return res;
        }
        let mut start = 0;
        let mut end = 8;
        while end <= s.len() {
            let num_str = &s[start..end];
            let num = i32::from_str_radix(num_str, 2).unwrap();
            if num == 0 {
                res.push("".into());
            } else {
                let str = &s[end..end + num as usize];
                res.push(str.to_string())
            }
            start = end + num as usize;
            end = start + 8;
        }

        res
    }
}