pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate: i32 = 0;
        let mut count: i32 = 0;

        for x in nums {
            if count == 0 {
                candidate = x;
            }
            if x == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate // guaranteed to exist
    }
}
