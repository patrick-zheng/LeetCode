pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 { return x; }

        let (mut lo, mut hi) = (1i32, x / 2);
        let mut ans = 0i32;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if mid <= x / mid {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans
    }
}
