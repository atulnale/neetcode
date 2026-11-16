impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut counter1 = 0;
        let mut counter2 = 0;
        let mut num1: Option<i32> = None;
        let mut num2: Option<i32> = None;

        for ele in nums.iter() {
            if counter1 == 0 && (num2.is_none() || (num2.is_some() && num2.unwrap() != *ele)) {
                counter1 = 1;
                num1 = Some(*ele);
            } else if counter2 == 0 && (num1.is_none() || (num1.is_some() && num1.unwrap() != *ele))
            {
                counter2 = 1;
                num2 = Some(*ele);
            } else if num1.is_some() && num1.unwrap() == *ele {
                counter1 += 1;
            } else if num2.is_some() && num2.unwrap() == *ele {
                counter2 += 1;
            } else {
                counter1 -= 1;
                counter2 -= 1;
            }
        }
        counter1 = 0;
        counter2 = 0;
        for ele in nums.iter() {
            if num1.is_some() && num1.unwrap() == *ele {
                counter1 += 1;
            } else if num2.is_some() && num2.unwrap() == *ele {
                counter2 += 1;
            }
        }
        let mut res = Vec::new();
        if counter1 > nums.len() / 3 {
            res.push(num1.unwrap());
        }
        if counter2 > nums.len() / 3 {
            res.push(num2.unwrap());
        }
        res
    }
}
