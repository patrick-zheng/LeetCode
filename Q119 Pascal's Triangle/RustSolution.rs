pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut row = vec![1; n + 1];

        for i in 1..n {
            for j in (1..=i).rev() {
                row[j] += row[j - 1];
            }
        }

        row
    }
}
