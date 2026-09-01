impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort();

        let mut t_chars: Vec<char> = t.chars().collect();
        t_chars.sort();
        let str_s: String = s_chars.iter().collect();
        let str_t: String = t_chars.iter().collect();
        str_s == str_t
    }
}
