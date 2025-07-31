pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = p.len() + 1;
        let n = s.len() + 1;
        let mut dp = vec![vec![false; n]; m];
        dp[0][0] = true;

        let p_bytes = p.as_bytes();
        let s_bytes = s.as_bytes();

        for i in 1..m {
            if p_bytes[i - 1] == b'*' {
                dp[i][0] = dp[i - 1][0];
            }
        }

        for i in 1..m {
            for j in 1..n {
                let pc = p_bytes[i - 1];
                let sc = s_bytes[j - 1];
                dp[i][j] = match pc {
                    b'?' => dp[i - 1][j - 1],
                    b'*' => dp[i - 1][j] || dp[i][j - 1],
                    _ => pc == sc && dp[i - 1][j - 1],
                };
            }
        }

        dp[m - 1][n - 1]
    }
}
