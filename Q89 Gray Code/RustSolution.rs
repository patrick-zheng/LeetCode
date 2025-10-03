pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let size = 1 << n;
        let mut res = Vec::with_capacity(size as usize);
        for i in 0..size {
            res.push(i ^ (i >> 1));
        }
        res
    }
}
