pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        fn dfs(start: i32, n: i32, k: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() == k {
                ans.push(path.clone());
                return;
            }
            let need = k - path.len();                // how many more we need
            let last_start = n - need as i32 + 1;     // pruning upper bound
            let mut i = start;
            while i <= last_start {
                path.push(i);
                dfs(i + 1, n, k, path, ans);
                path.pop();
                i += 1;
            }
        }
        if k == 0 {
            ans.push(Vec::new());
            return ans;
        }
        if k as i32 > n {
            return ans;
        }
        dfs(1, n, k as usize, &mut path, &mut ans);
        ans
    }
}
