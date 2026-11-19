impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut l, mut r) = (0usize, height.len() - 1);
        let (mut lmax, mut rmax) = (height[0], height[height.len() - 1]);
        while l < r {
            if height[l] < height[r] {
                l += 1;
                lmax = lmax.max(height[l]);
                ans += lmax - height[l];
            } else {
                r -= 1;
                rmax = rmax.max(height[r]);
                ans += rmax - height[r];
            }
        }
        ans
    }
}