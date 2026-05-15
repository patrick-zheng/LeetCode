use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut memo = HashMap::new();
        Self::dfs(&expression, &mut memo)
    }

    fn dfs(s: &str, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(v) = memo.get(s) {
            return v.clone();
        }
        let mut res = Vec::new();
        let mut has_op = false;
        for (i, c) in s.char_indices() {
            if c != '+' && c != '-' && c != '*' {
                continue;
            }
            has_op = true;
            let left = Self::dfs(&s[..i], memo);
            let right = Self::dfs(&s[i + c.len_utf8()..], memo);
            for &a in &left {
                for &b in &right {
                    res.push(match c {
                        '+' => a + b,
                        '-' => a - b,
                        _ => a * b,
                    });
                }
            }
        }
        if !has_op {
            res.push(s.parse::<i32>().unwrap());
        }
        memo.insert(s.to_string(), res.clone());
        res
    }
}
