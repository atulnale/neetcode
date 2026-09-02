impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let right = nums.len() - 1;
        Self::merge_sort(&mut nums, 0, right);
        return nums;
    }
    fn merge_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if left >= right {
            return;
        }
        let mid = left + ((right - left) / 2);
        Self::merge_sort(nums, left, mid);
        Self::merge_sort(nums, mid + 1, right);
        Self::merge(nums, left, mid, right);
    }

    fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
        let mut arr: Vec<i32> = Vec::new();
        let mut i = left;
        let mut j = mid+1;
        while i <= mid || j <= right {
            let num1 = if i > mid { i32::MAX } else { nums[i] };

            let num2 = if j > right { i32::MAX } else { nums[j] };
            if num1 < num2 {
                arr.push(num1);
                i += 1;
            } else {
                arr.push(num2);
                j += 1;
            }
        }
         i = left;
        for ele in arr {
            nums[i] = ele;
            i += 1;
        }
    }
}
