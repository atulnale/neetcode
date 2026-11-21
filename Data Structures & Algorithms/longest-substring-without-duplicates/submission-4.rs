impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [0usize; 128];
        let mut left = 0usize;
        let mut longest = 0usize;

        for (right, byte) in s.bytes().enumerate() {
            let index = byte as usize;
            
            // positions are sotred as index + 1; zero means unsseen
            left = left.max(last_seen[index]);
            longest = longest.max(right - left + 1);
            last_seen[index] = right + 1;
        }

        longest as i32
    }

}
