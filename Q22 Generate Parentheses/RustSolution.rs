pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let n = n as usize;  // Convert i32 to usize for indexing purposes

        // Helper function for backtracking
        fn backtrack(current: String, open_count: usize, close_count: usize, n: usize, result: &mut Vec<String>) {
            // If the current string has reached the correct length, add it to the result
            if current.len() == 2 * n {
                result.push(current);
                return;
            }

            // Add an open parenthesis if we haven't reached the limit
            if open_count < n {
                backtrack(current.clone() + "(", open_count + 1, close_count, n, result);
            }

            // Add a close parenthesis if it doesn't exceed the number of open ones
            if close_count < open_count {
                backtrack(current.clone() + ")", open_count, close_count + 1, n, result);
            }
        }

        // Start the backtracking process
        backtrack("".to_string(), 0, 0, n, &mut result);

        result
    }
}
