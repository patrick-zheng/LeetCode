pub struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        let mut used = vec![false; nums.len()];
        let mut path = Vec::new();

        fn backtrack(nums: &Vec<i32>, path: &mut Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                res.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] && !used[i - 1] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                backtrack(nums, path, used, res);
                path.pop();
                used[i] = false;
            }
        }

        backtrack(&nums, &mut path, &mut used, &mut res);
        res
    }
}
