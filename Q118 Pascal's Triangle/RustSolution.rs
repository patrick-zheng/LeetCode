pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut tri: Vec<Vec<i32>> = Vec::with_capacity(n);
        for r in 0..n {
            if r == 0 {
                tri.push(vec![1]);
            } else {
                let prev = &tri[r - 1];
                // Sum adjacent pairs: prev[i] + prev[i+1]
                let mid: Vec<i32> = prev.windows(2).map(|w| w[0] + w[1]).collect();
                let mut row = Vec::with_capacity(prev.len() + 1);
                row.push(1);
                row.extend(mid);
                row.push(1);
                tri.push(row);
            }
        }
        tri
    }
}
