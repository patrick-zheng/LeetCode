pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n = n as i32;
        let mut res = Vec::new();
        let mut path = Vec::with_capacity(k);
        Self::dfs(1, k, n, &mut path, &mut res);
        res
    }

    fn min_sum_k(start: i32, k_left: usize) -> i32 {
        let kk = k_left as i32;
        kk * start + kk * (kk - 1) / 2
    }

    fn max_sum_k(start: i32, k_left: usize) -> i32 {
        let mut total = 0;
        let mut x = 9;
        for _ in 0..k_left {
            if x < start {
                return -1;
            }
            total += x;
            x -= 1;
        }
        total
    }

    fn dfs(start: i32, k_left: usize, target: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k_left == 0 {
            if target == 0 {
                res.push(path.clone());
            }
            return;
        }
        if start > 9 || target <= 0 {
            return;
        }
        if ((10 - start) as usize) < k_left {
            return;
        }

        let lo = Self::min_sum_k(start, k_left);
        let hi = Self::max_sum_k(start, k_left);
        if hi < 0 || target < lo || target > hi {
            return;
        }

        for i in start..=9 {
            if i > target {
                break;
            }
            path.push(i);
            Self::dfs(i + 1, k_left - 1, target - i, path, res);
            path.pop();
        }
    }
}
