use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if n != s2.len() { return false; }
        if s1 == s2 { return true; }

        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();

        // Build prefix counts for O(26) multiset checks
        fn build_pref(bytes: &[u8]) -> Vec<[i32; 26]> {
            let n = bytes.len();
            let mut pref = vec![[0; 26]; n + 1];
            for i in 0..n {
                pref[i + 1] = pref[i];
                let idx = (bytes[i] - b'a') as usize;
                pref[i + 1][idx] += 1;
            }
            pref
        }

        let pref1 = build_pref(b1);
        let pref2 = build_pref(b2);

        let mut memo: HashMap<(usize, usize, usize), bool> = HashMap::new();

        // Multiset pruning using prefix counts
        let same_multiset = |i: usize, j: usize, len: usize| -> bool {
            for k in 0..26 {
                let c1 = pref1[i + len][k] - pref1[i][k];
                let c2 = pref2[j + len][k] - pref2[j][k];
                if c1 != c2 { return false; }
            }
            true
        };

        fn dfs<'a>(
            b1: &'a [u8],
            b2: &'a [u8],
            memo: &mut HashMap<(usize, usize, usize), bool>,
            same_multiset: &impl Fn(usize, usize, usize) -> bool,
            i: usize,
            j: usize,
            len: usize,
        ) -> bool {
            if let Some(&ans) = memo.get(&(i, j, len)) {
                return ans;
            }

            // Exact substring equality short-circuit
            if &b1[i..i + len] == &b2[j..j + len] {
                memo.insert((i, j, len), true);
                return true;
            }

            // Character multiset pruning
            if !same_multiset(i, j, len) {
                memo.insert((i, j, len), false);
                return false;
            }

            for k in 1..len {
                // No swap
                if dfs(b1, b2, memo, same_multiset, i, j, k)
                    && dfs(b1, b2, memo, same_multiset, i + k, j + k, len - k)
                {
                    memo.insert((i, j, len), true);
                    return true;
                }
                // Swap
                if dfs(b1, b2, memo, same_multiset, i, j + len - k, k)
                    && dfs(b1, b2, memo, same_multiset, i + k, j, len - k)
                {
                    memo.insert((i, j, len), true);
                    return true;
                }
            }

            memo.insert((i, j, len), false);
            false
        }

        dfs(b1, b2, &mut memo, &same_multiset, 0, 0, n)
    }
}