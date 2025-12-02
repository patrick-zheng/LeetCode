pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }

        // Work on chars to avoid UTF-8 indexing issues
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        // pal[i][j] == true if chars[i..=j] is a palindrome
        let mut pal = vec![vec![false; n]; n];

        // Fill palindrome table
        for i in (0..n).rev() {
            for j in i..n {
                if chars[i] == chars[j] && (j - i < 2 || pal[i + 1][j - 1]) {
                    pal[i][j] = true;
                }
            }
        }

        // dp[i] = min cuts for s[0..=i]
        let mut dp = vec![0_i32; n];

        for i in 0..n {
            let mut min_cuts = i as i32; // worst case
            if pal[0][i] {
                min_cuts = 0;
            } else {
                for j in 0..i {
                    if pal[j + 1][i] {
                        min_cuts = min_cuts.min(dp[j] + 1);
                    }
                }
            }
            dp[i] = min_cuts;
        }

        dp[n - 1]
    }
}
