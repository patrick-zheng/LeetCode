pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        // Step 1: Preprocess string with boundaries
        let mut t = "^#".to_string();
        for ch in s.chars() {
            t.push(ch);
            t.push('#');
        }
        t.push('$');

        let chars: Vec<char> = t.chars().collect();
        let n = chars.len();
        let mut p = vec![0; n];
        let (mut center, mut right) = (0, 0);

        // Step 2: Manacher’s algorithm
        for i in 1..n - 1 {
            let mirror = 2 * center - i;

            if i < right {
                p[i] = p[mirror].min(right - i);
            }

            // Expand around center i
            while chars[i + p[i] + 1] == chars[i - p[i] - 1] {
                p[i] += 1;
            }

            if i + p[i] > right {
                center = i;
                right = i + p[i];
            }
        }

        // Step 3: Find the longest palindrome
        let (mut max_len, mut center_index) = (0, 0);
        for (i, &len) in p.iter().enumerate() {
            if len > max_len {
                max_len = len;
                center_index = i;
            }
        }

        let start = (center_index - max_len) / 2;
        s[start..start + max_len].to_string()
    }
}

