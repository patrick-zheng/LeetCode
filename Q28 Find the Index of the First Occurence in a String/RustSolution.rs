pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let h: Vec<char> = haystack.chars().collect();
        let n: Vec<char> = needle.chars().collect();
        let lps = build_lps(&n);

        let (mut i, mut j) = (0, 0);
        while i < h.len() {
            if h[i] == n[j] {
                i += 1;
                j += 1;
                if j == n.len() {
                    return (i - j) as i32;
                }
            } else if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
        -1
    }
}

fn build_lps(pattern: &[char]) -> Vec<usize> {
    let mut lps = vec![0; pattern.len()];
    let mut len = 0;
    let mut i = 1;

    while i < pattern.len() {
        if pattern[i] == pattern[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else if len != 0 {
            len = lps[len - 1];
        } else {
            lps[i] = 0;
            i += 1;
        }
    }
    lps
}
