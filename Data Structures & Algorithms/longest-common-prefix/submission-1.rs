impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs.first().map(String::as_str).unwrap_or_default();

        for str in strs.iter().skip(1) {
            let mut end = prefix.len().min(str.len());

            for ((i, left), (_, right)) in prefix.char_indices().zip(str.char_indices()) {
                if left != right {
                    end = i;
                    break;
                }
            }
            prefix = &prefix[..end];
        }

        prefix.to_string()
    }
}
