pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();

        // Immutable-style recursion: pass/return owned vectors
        fn dfs(start: usize, nums: &Vec<i32>, path: Vec<i32>, mut res: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            res.push(path.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] {
                    continue; // skip duplicates at this depth
                }
                let mut next = path.clone();
                next.push(nums[i]);
                res = dfs(i + 1, nums, next, res);
            }
            res
        }

        dfs(0, &sorted, Vec::new(), Vec::new())
    }
}
