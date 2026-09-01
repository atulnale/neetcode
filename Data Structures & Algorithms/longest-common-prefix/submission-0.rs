impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
if strs.len() == 0 {
            return "".to_string();
        };
        let first = strs[0].as_bytes();
        let mut end = first.len();
        for i in 1..strs.len() {
            end = end.min(strs[i].len());
            let cmp = strs[i].as_bytes();
            for j in 0..end {
                if first[j] != cmp[j] {
                    end = j;
                    break;
                }
            }
        }
        let str = first[0..end].to_vec();
        String::from_utf8(str).unwrap()
    }
}
