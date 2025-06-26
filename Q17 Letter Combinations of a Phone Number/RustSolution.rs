pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let digit_to_letters: std::collections::HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ].iter().cloned().collect();

        let mut result = Vec::new();

        // Helper function for backtracking
        fn backtrack(
            digits: &str,
            index: usize,
            path: &mut Vec<char>,
            result: &mut Vec<String>,
            digit_to_letters: &std::collections::HashMap<char, Vec<char>>
        ) {
            if index == digits.len() {
                result.push(path.iter().collect());
                return;
            }

            if let Some(letters) = digit_to_letters.get(&digits[index..index+1].chars().next().unwrap()) {
                for &letter in letters {
                    path.push(letter);
                    backtrack(digits, index + 1, path, result, digit_to_letters);
                    path.pop();
                }
            }
        }

        backtrack(&digits, 0, &mut Vec::new(), &mut result, &digit_to_letters);
        result
    }
}
