impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();

        let mut iter1 = word1.chars().peekable();
        let mut iter2 = word2.chars().peekable();
        while let (Some(a), Some(b)) = (iter1.peek(), iter2.peek()) {
            res.push(*a);
            res.push(*b);
            iter1.next();
            iter2.next();
        }
        res.extend(iter1);
        res.extend(iter2);

        res
    }
}