impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut l = 0usize;
        let mut r = arr.len() - 1;
        while (1 + r - l) as i32 != k {
            if (arr[l] - x).abs() == (arr[r] - x).abs() {
                r -= 1;
            } else if (arr[l] - x).abs() < (arr[r] - x).abs() {
                r -= 1;
            } else {
                l += 1;
            }
        }
        arr[l..=r].to_vec()
    }
}
