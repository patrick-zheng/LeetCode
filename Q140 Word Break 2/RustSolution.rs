use std::collections::{HashMap, HashSet};

public struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let bytes = s.as_bytes();
        let max_len = dict.iter().map(|w| w.len()).max().unwrap_or(0);

        // prune: can[i] = whether s[i:] is breakable at all
        let mut can = vec![false; n + 1];
        can[n] = true;
        for i in (0..n).rev() {
            let end = (i + max_len).min(n);
            let mut ok = false;
            for j in (i + 1)..=end {
                if can[j] {
                    let w = std::str::from_utf8(&bytes[i..j]).unwrap();
                    if dict.contains(w) {
                        ok = true;
                        break;
                    }
                }
            }
            can[i] = ok;
        }

        fn dfs(
            i: usize,
            bytes: &[u8],
            dict: &HashSet<String>,
            can: &[bool],
            max_len: usize,
            memo: &mut HashMap<usize, Vec<String>>,
        ) -> Vec<String> {
            if !can[i] {
                return vec![];
            }
            if i == bytes.len() {
                return vec![String::new()];
            }
            if let Some(v) = memo.get(&i) {
                return v.clone();
            }

            let n = bytes.len();
            let end = (i + max_len).min(n);
            let mut res: Vec<String> = Vec::new();

            for j in (i + 1)..=end {
                if !can[j] { continue; }
                let w = std::str::from_utf8(&bytes[i..j]).unwrap().to_string();
                if dict.contains(&w) {
                    let tails = dfs(j, bytes, dict, can, max_len, memo);
                    for t in tails {
                        if t.is_empty() {
                            res.push(w.clone());
                        } else {
                            res.push(format!("{} {}", w, t));
                        }
                    }
                }
            }

            memo.insert(i, res.clone());
            res
        }

        let mut memo: HashMap<usize, Vec<String>> = HashMap::new();
        dfs(0, bytes, &dict, &can, max_len, &mut memo)
    }
}
