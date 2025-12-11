use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let word_set: HashSet<String> = word_dict.into_iter().collect();

        // dp[i] = true if s[0..i] can be segmented
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        // Optional optimization: max word length
        let max_len = word_set
            .iter()
            .map(|w| w.len())
            .max()
            .unwrap_or(0);

        // Work with byte indices (OK for LeetCode: lowercase English letters)
        for i in 1..=n {
            let start = if i > max_len { i - max_len } else { 0 };
            for j in start..i {
                if dp[j] && word_set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}
