use std::collections::{BinaryHeap, HashMap};


#[derive(PartialEq, Eq)]
struct Node {
    value: i32,
    count: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max_heap: BinaryHeap<Node> = BinaryHeap::new();
        let mut map: HashMap<i32, usize> = HashMap::new();
        for ele in nums {
            map.insert(ele, map.get(&ele).unwrap_or(&0) + 1);
        }
        for (value, count) in map.iter() {
            max_heap.push(Node {
                count: *count,
                value: *value,
            });
        }
        let mut res = Vec::new();
        for i in 0..k {
            let ele = max_heap.pop().unwrap_or(Node { value: 0, count: 0 });
            res.push(ele.value);
        }
        res
    }
}