impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }
        let x = x as i64;
        let (mut l, mut r) = (0, x);
        while l < r {
            let mid = l + (r - l) / 2;
            let square = mid * mid;
            if square == x {
                return mid as i32;
            } else if square < x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }

        r as i32 - 1
    }
}