pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let (mut a, mut b) = (1, 2); // f(1), f(2)
        for _ in 3..=n {
            let c = a + b; // f(i) = f(i-1) + f(i-2)
            a = b;
            b = c;
        }
        b
    }
}
