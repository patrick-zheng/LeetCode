pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut heights = vec![0i32; cols + 1]; // +1 sentinel zero
        let mut best = 0i32;

        for r in 0..rows {
            // Update histogram for this row
            for c in 0..cols {
                heights[c] = if matrix[r][c] == '1' { heights[c] + 1 } else { 0 };
            }

            // Largest Rectangle in Histogram with monotonic increasing stack of indices
            let mut st: Vec<usize> = Vec::new();
            for i in 0..=cols {
                while let Some(&top) = st.last() {
                    if heights[i] < heights[top] {
                        let h = heights[top];
                        st.pop();
                        let left = st.last().copied();
                        let width = (i as i32) - (left.map(|x| x as i32).unwrap_or(-1)) - 1;
                        best = best.max(h * width);
                    } else {
                        break;
                    }
                }
                st.push(i);
            }
        }
        best
    }
}
