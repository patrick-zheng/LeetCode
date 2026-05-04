pub struct Solution;

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let n = n as i64;
        let mut count: i64 = 0;
        let mut factor: i64 = 1;

        while factor <= n {
            let higher = n / (factor * 10);
            let current = (n / factor) % 10;
            let lower = n % factor;

            count += if current == 0 {
                higher * factor
            } else if current == 1 {
                higher * factor + lower + 1
            } else {
                (higher + 1) * factor
            };

            factor *= 10;
        }

        count as i32
    }
}
