pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut h = heights;
        h.push(0);
        let mut stack: Vec<usize> = Vec::new();
        let mut best: i64 = 0;

        for i in 0..h.len() {
            while let Some(&top) = stack.last() {
                if h[top] > h[i] {
                    let height = h[stack.pop().unwrap()] as i64;
                    let left = stack.last().copied().map(|idx| idx as i64).unwrap_or(-1);
                    let width = i as i64 - left - 1;
                    best = best.max(height * width);
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        best as i32
    }
}
