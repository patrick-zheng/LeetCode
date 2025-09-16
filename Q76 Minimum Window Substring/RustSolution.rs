use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() || s.is_empty() || t.chars().count() > s.chars().count() {
            return String::new();
        }

        // Count required frequencies for t
        let mut need: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *need.entry(c).or_insert(0) += 1;
        }
        let required = need.len(); // number of distinct chars to satisfy

        let s_chars: Vec<char> = s.chars().collect();
        let mut window: HashMap<char, i32> = HashMap::new();
        let mut formed = 0usize;

        let (mut best_len, mut best_l, mut best_r) = (usize::MAX, 0usize, 0usize);
        let (mut l, mut r) = (0usize, 0usize);

        while r < s_chars.len() {
            let cr = s_chars[r];
            let count = window.entry(cr).or_insert(0);
            *count += 1;

            if let Some(&need_r) = need.get(&cr) {
                if *count == need_r {
                    formed += 1;
                }
            }

            // Try to shrink from the left while valid
            while formed == required && l <= r {
                if r - l + 1 < best_len {
                    best_len = r - l + 1;
                    best_l = l;
                    best_r = r;
                }

                let cl = s_chars[l];
                if let Some(c) = window.get_mut(&cl) {
                    *c -= 1;
                    if let Some(&need_l) = need.get(&cl) {
                        if *c < need_l {
                            formed -= 1;
                        }
                    }
                }
                l += 1;
            }

            r += 1;
        }

        if best_len == usize::MAX {
            String::new()
        } else {
            s_chars[best_l..=best_r].iter().collect()
        }
    }
}
