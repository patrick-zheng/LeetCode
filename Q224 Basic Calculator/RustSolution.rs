pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut res: i64 = 0;
        let mut num: i64 = 0;
        let mut sign: i64 = 1;
        let mut stack: Vec<i64> = Vec::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as i64 - b'0' as i64);
            } else if c == '+' {
                res += sign * num;
                num = 0;
                sign = 1;
            } else if c == '-' {
                res += sign * num;
                num = 0;
                sign = -1;
            } else if c == '(' {
                stack.push(res);
                stack.push(sign);
                res = 0;
                sign = 1;
            } else if c == ')' {
                res += sign * num;
                num = 0;
                let prev_sign = stack.pop().unwrap();
                let prev_res = stack.pop().unwrap();
                res = prev_res + prev_sign * res;
            }
        }

        (res + sign * num) as i32
    }
}
