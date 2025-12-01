pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut is_pal = vec![vec![false; n]; n];

        // Precompute palindromes: is_pal[i][j] = true if chars[i..=j] is palindrome
        for i in (0..n).rev() {
            for j in i..n {
                if chars[i] == chars[j] && (j - i <= 2 || is_pal[i + 1][j - 1]) {
                    is_pal[i][j] = true;
                }
            }
        }

        let mut res: Vec<Vec<String>> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        fn backtrack(
            start: usize,
            chars: &Vec<char>,
            is_pal: &Vec<Vec<bool>>,
            path: &mut Vec<String>,
            res: &mut Vec<Vec<String>>,
        ) {
            let n = chars.len();
            if start == n {
                res.push(path.clone());
                return;
            }

            for end in start..n {
                if is_pal[start][end] {
                    let substr: String = chars[start..=end].iter().collect();
                    path.push(substr);
                    backtrack(end + 1, chars, is_pal, path, res);
                    path.pop();
                }
            }
        }

        backtrack(0, &chars, &is_pal, &mut path, &mut res);
        res
    }
}
