impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let arr: Vec<char> = s.chars().collect();
        let mut left = 0usize;
        let mut right = arr.len() - 1;

        while left < right {
            if !arr[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !arr[right].is_alphanumeric() {
                right -= 1;
                continue;
            }
            if !arr[left].eq_ignore_ascii_case(&arr[right]) {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}