pub struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;

        // Available digits
        let mut nums: Vec<i32> = (1..=n as i32).collect();

        // Factorials
        let mut fact = vec![1usize; n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * i;
        }

        // 0-based k
        let mut k = (k as isize - 1) as usize;

        let mut out = String::with_capacity(n);
        for i in (1..=n).rev() {
            let block = fact[i - 1];
            let idx = k / block;
            out.push_str(&nums[idx].to_string());
            nums.remove(idx);
            k %= block;
        }
        out
    }
}
