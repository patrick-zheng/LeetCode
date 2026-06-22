use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut left_remove = 0i32;
        let mut right_remove = 0i32;
        for &ch in s {
            if ch == b'(' {
                left_remove += 1;
            } else if ch == b')' {
                if left_remove > 0 {
                    left_remove -= 1;
                } else {
                    right_remove += 1;
                }
            }
        }

        let mut result = HashSet::new();
        let mut path = Vec::new();

        fn dfs(
            s: &[u8],
            index: usize,
            open_count: i32,
            close_count: i32,
            left_rem: i32,
            right_rem: i32,
            path: &mut Vec<u8>,
            result: &mut HashSet<String>,
        ) {
            if index == s.len() {
                if left_rem == 0 && right_rem == 0 {
                    result.insert(String::from_utf8(path.clone()).unwrap());
                }
                return;
            }

            let ch = s[index];
            if ch != b'(' && ch != b')' {
                path.push(ch);
                dfs(
                    s,
                    index + 1,
                    open_count,
                    close_count,
                    left_rem,
                    right_rem,
                    path,
                    result,
                );
                path.pop();
                return;
            }

            if ch == b'(' && left_rem > 0 {
                dfs(
                    s,
                    index + 1,
                    open_count,
                    close_count,
                    left_rem - 1,
                    right_rem,
                    path,
                    result,
                );
            } else if ch == b')' && right_rem > 0 {
                dfs(
                    s,
                    index + 1,
                    open_count,
                    close_count,
                    left_rem,
                    right_rem - 1,
                    path,
                    result,
                );
            }

            if ch == b'(' {
                path.push(ch);
                dfs(
                    s,
                    index + 1,
                    open_count + 1,
                    close_count,
                    left_rem,
                    right_rem,
                    path,
                    result,
                );
                path.pop();
            } else if close_count < open_count {
                path.push(ch);
                dfs(
                    s,
                    index + 1,
                    open_count,
                    close_count + 1,
                    left_rem,
                    right_rem,
                    path,
                    result,
                );
                path.pop();
            }
        }

        dfs(
            s,
            0,
            0,
            0,
            left_remove,
            right_remove,
            &mut path,
            &mut result,
        );
        result.into_iter().collect()
    }
}
