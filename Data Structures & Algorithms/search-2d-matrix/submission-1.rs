impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut row: usize = 0;

        let mut l = 0usize;
        let mut r = rows - 1;
        let mut found = false;
        while l <= r {
            let mid = l + (r - l) / 2;
            if target >= matrix[mid][0] && target <= matrix[mid][cols - 1] {
                row = mid;
                found = true;
                break;
            } else if target >= matrix[mid][0] {
                l = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                r = mid - 1;
            }
        }
        if !found {
            return false;
        }
        l = 0;
        r = cols - 1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if target == matrix[row][mid] {
                return true;
            } else if target > matrix[row][mid] {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        false
    }
}