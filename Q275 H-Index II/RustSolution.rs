pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len() as i32;
        let mut left = 0i32;
        let mut right = n - 1;
        let mut answer = 0i32;
        while left <= right {
            let mid = left + (right - left) / 2;
            let h = n - mid;
            if citations[mid as usize] >= h {
                answer = h;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        answer
    }
}
