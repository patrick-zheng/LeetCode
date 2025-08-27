pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let total: u128 = (m as u128 + n as u128 - 2) as u128;
        let k: u128 = ((m - 1) as u128).min((n - 1) as u128);
        let mut res: u128 = 1;

        for i in 1..=k {
            let num = total - (k - i);
            res = res * num / i; // exact for binomial coefficients
        }
        res as i32
    }
}
