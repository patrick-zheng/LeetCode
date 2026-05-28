pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut ugly = vec![0; n];
        ugly[0] = 1;
        let mut i2 = 0usize;
        let mut i3 = 0usize;
        let mut i5 = 0usize;
        for i in 1..n {
            let next = (ugly[i2] * 2).min(ugly[i3] * 3).min(ugly[i5] * 5);
            ugly[i] = next;
            if next == ugly[i2] * 2 {
                i2 += 1;
            }
            if next == ugly[i3] * 3 {
                i3 += 1;
            }
            if next == ugly[i5] * 5 {
                i5 += 1;
            }
        }
        ugly[n - 1]
    }
}
