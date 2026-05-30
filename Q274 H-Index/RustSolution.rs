pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut bucket = vec![0i32; n + 1];
        for &c in &citations {
            let idx = if c as usize >= n { n } else { c as usize };
            bucket[idx] += 1;
        }

        let mut papers = 0i32;
        for h in (0..=n).rev() {
            papers += bucket[h];
            if papers >= h as i32 {
                return h as i32;
            }
        }
        0
    }
}
