
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    num: i32,
    count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.count.cmp(&other.count))
    }
}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, Node> = HashMap::new();
        let mut heap = BinaryHeap::new();
        for i in nums.iter() {
            let count = map.entry(*i).or_insert(Node { num: *i, count: 0 }).count + 1;
            map.insert(*i, Node { num: *i, count });
        }

        for (_, node) in map.iter() {
            heap.push(node);
        }
        let mut res = Vec::new();
        for i in 0..k {
            if let Some(node) = heap.pop() {
                res.push(node.num);
            }
        }
        res
    }
}