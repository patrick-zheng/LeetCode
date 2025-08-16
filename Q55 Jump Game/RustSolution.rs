pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut goal = nums.len() as i32 - 1;

        for i in (0..nums.len() - 1).rev() {
            if i as i32 + nums[i] >= goal {
                goal = i as i32;
            }
        }

        goal == 0
    }
}
