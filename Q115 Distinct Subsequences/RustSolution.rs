pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = s.len();
        let n = t.len();
        if n == 0 { return 1; }
        if m < n { return 0; }

        let t_chars: Vec<char> = t.chars().collect();
        let mut dp: Vec<i64> = vec![0; n + 1];
        dp[0] = 1;

        for ch in s.chars() {
            for j in (1..=n).rev() {
                if ch == t_chars[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n] as i32
    }
}
