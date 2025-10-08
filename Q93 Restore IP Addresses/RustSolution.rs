pub struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs(i: usize, parts: &mut Vec<String>, s: &str, res: &mut Vec<String>) {
            // If we already chose 4 parts, the string must be fully consumed
            if parts.len() == 4 {
                if i == s.len() {
                    res.push(parts.join("."));
                }
                return;
            }

            // Prune by remaining-length constraints:
            // remaining chars must be between (4 - parts.len()) and 3*(4 - parts.len())
            let remaining = s.len().saturating_sub(i);
            let need = 4 - parts.len();
            if remaining < need || remaining > 3 * need {
                return;
            }

            // Try segment lengths 1..=3
            for len in 1..=3 {
                if i + len > s.len() {
                    break;
                }

                // Leading zero rule: "0" is okay, but "01", "00", ... are not
                if len > 1 && &s[i..i + 1] == "0" {
                    break;
                }

                let seg = &s[i..i + len];

                // Value must be <= 255
                // (safe to parse: seg is 1..=3 ASCII digits)
                let val: u16 = seg.parse().unwrap();
                if val > 255 {
                    continue;
                }

                parts.push(seg.to_string());
                dfs(i + len, parts, s, res);
                parts.pop();
            }
        }

        // Quick rejects
        if s.len() < 4 || s.len() > 12 {
            return Vec::new();
        }

        let mut res = Vec::new();
        let mut parts = Vec::with_capacity(4);
        dfs(0, &mut parts, &s, &mut res);
        res
    }
}
