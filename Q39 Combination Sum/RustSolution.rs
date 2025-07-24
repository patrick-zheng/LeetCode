pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            candidates: &Vec<i32>,
            start: usize,
            path: &mut Vec<i32>,
            remaining: i32,
            result: &mut Vec<Vec<i32>>,
        ) {
            if remaining == 0 {
                result.push(path.clone());
                return;
            }
            if remaining < 0 {
                return;
            }

            for i in start..candidates.len() {
                path.push(candidates[i]);
                backtrack(candidates, i, path, remaining - candidates[i], result);
                path.pop(); // backtrack
            }
        }

        let mut result = Vec::new();
        let mut path = Vec::new();
        backtrack(&candidates, 0, &mut path, target, &mut result);
        result
    }
}
