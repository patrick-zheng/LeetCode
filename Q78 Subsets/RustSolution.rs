pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut current: Vec<i32> = Vec::new();
        Self::backtrack(0, &nums, &mut current, &mut result);
        result
    }

    fn backtrack(index: usize, nums: &Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(current.clone());
            return;
        }

        // Exclude nums[index]
        Self::backtrack(index + 1, nums, current, result);

        // Include nums[index]
        current.push(nums[index]);
        Self::backtrack(index + 1, nums, current, result);
        current.pop();
    }
}
