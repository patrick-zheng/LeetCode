pub struct Solution;

impl Solution {
    fn next_num(mut n: i32) -> i32 {
        let mut total = 0;
        while n > 0 {
            let digit = n % 10;
            total += digit * digit;
            n /= 10;
        }
        total
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::next_num(n);

        while fast != 1 && slow != fast {
            slow = Self::next_num(slow);
            fast = Self::next_num(Self::next_num(fast));
        }

        fast == 1
    }
}
