pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (mut a, mut b) = (s1, s2);
        if a.len() + b.len() != s3.len() {
            return false;
        }
        // Ensure b is the shorter to minimize space
        if b.len() > a.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let (a_b, b_b, c_b) = (a.as_bytes(), b.as_bytes(), s3.as_bytes());
        let n = a_b.len();
        let m = b_b.len();

        let mut dp = vec![false; m + 1];
        dp[0] = true;

        // Initialize first row using only b
        for j in 1..=m {
            dp[j] = dp[j - 1] && b_b[j - 1] == c_b[j - 1];
        }

        for i in 1..=n {
            // First column using only a
            dp[0] = dp[0] && a_b[i - 1] == c_b[i - 1];
            for j in 1..=m {
                let take_a = dp[j] && a_b[i - 1] == c_b[i + j - 1];
                let take_b = dp[j - 1] && b_b[j - 1] == c_b[i + j - 1];
                dp[j] = take_a || take_b;
            }
        }

        dp[m]
    }
}
