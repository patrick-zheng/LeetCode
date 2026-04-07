pub struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let n = s.len();
        let rev: String = s.chars().rev().collect();
        let combined = format!("{}#{}", s, rev);
        let b = combined.as_bytes();
        let m = b.len();
        let mut lps = vec![0usize; m];
        for i in 1..m {
            let mut j = lps[i - 1];
            while j > 0 && b[i] != b[j] {
                j = lps[j - 1];
            }
            if b[i] == b[j] {
                j += 1;
            }
            lps[i] = j;
        }
        let l = lps[m - 1];
        format!("{}{}", &rev[..n - l], s)
    }
}
