impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, 1);
        let mut ans = 0;

        while r < prices.len() {
            if prices[r] > prices[l] {
                ans = ans.max(prices[r] - prices[l]);
            } else {
                l = r;
            }
             r += 1;
        }

        ans
    }
}