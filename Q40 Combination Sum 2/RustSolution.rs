pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable(); // Faster and sufficient for integers
        let mut res = Vec::new();
        let mut path = Vec::with_capacity(candidates.len());
        Self::backtrack(&candidates, target, 0, 0, &mut path, &mut res);
        res
    }

    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        sum: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            res.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            let curr = candidates[i];

            // Skip duplicates at the same recursion level
            if i > start && curr == candidates[i - 1] {
                continue;
            }

            if sum + curr > target {
                break;
            }

            path.push(curr);
            Self::backtrack(candidates, target, i + 1, sum + curr, path, res);
            path.pop();
        }
    }
}
