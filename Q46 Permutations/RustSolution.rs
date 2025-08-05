pub struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();  // Start from the lexicographically smallest permutation
        let mut result = vec![nums.clone()];

        while Solution::next_permutation(&mut nums) {
            result.push(nums.clone());
        }

        result
    }

    fn next_permutation(nums: &mut Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 { return false; }

        let mut i = n - 2;
        while i != usize::MAX && nums[i] >= nums[i + 1] {
            if i == 0 { break; }
            i -= 1;
        }

        if i == 0 && nums[i] >= nums[i + 1] {
            return false;  // Already the last permutation
        }

        let mut j = n - 1;
        while nums[j] <= nums[i] {
            j -= 1;
        }

        nums.swap(i, j);
        nums[i + 1..].reverse();
        true
    }
}
