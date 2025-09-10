pub struct Solution;

impl Solution {
    pub fn functionName() -> void {
        let (a, b) = if word1.len() >= word2.len() {
            (word1, word2)
        } else {
            (word2, word1)
        };

        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();

        let mut dp: Vec<i32> = (0..=n as i32).collect(); // dp[j] = j

        for i in 1..=m {
            let mut prev_diag = dp[0]; // dp[i-1][0]
            dp[0] = i as i32;          // delete i chars

            for j in 1..=n {
                let temp = dp[j]; // old dp[i-1][j]
                if a_chars[i - 1] == b_chars[j - 1] {
                    dp[j] = prev_diag;
                } else {
                    dp[j] = 1 + (prev_diag.min(temp).min(dp[j - 1]));
                }
                prev_diag = temp;
            }
        }
        dp[n]
    }
}
