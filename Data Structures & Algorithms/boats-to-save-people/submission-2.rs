impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let mut left: usize = 0usize;
        let mut right: i32 = people.len() as i32 - 1;
        let mut ans = 0;
        while left as i32 <= right {
            let limit = limit -  people[right as usize];
            if (left < right as  usize) && people[left] <= limit {
                left += 1
            }         
            right -= 1;
            ans += 1;
        }
        ans
    }
}