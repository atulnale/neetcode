impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let arr: Vec<char> = s.chars().collect();
        let mut is_deleted = false;

        let mut left = 0usize;
        let mut right = arr.len() - 1;
        return Self::helper(&arr, left, right, is_deleted);
    }
    fn helper(arr: &Vec<char>, mut left: usize, mut right: usize, mut is_deleted: bool) -> bool {
        while left < right {
            if arr[left] != arr[right] {
                if is_deleted {
                    return false;
                }
                return Self::helper(&arr, left + 1, right, true) || Self::helper(&arr, left, right - 1, true);
            }
            left += 1;
            right -= 1;
        }
        true
    }
}