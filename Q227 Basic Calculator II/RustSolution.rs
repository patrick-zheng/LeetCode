pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result: i64 = 0;
        let mut last_number: i64 = 0;
        let mut curr: i64 = 0;
        let mut op = b'+';

        for i in 0..s.len() {
            let c = s[i];
            if c.is_ascii_digit() {
                curr = curr * 10 + i64::from(c - b'0');
            }
            if c == b'+' || c == b'-' || c == b'*' || c == b'/' || i == s.len() - 1 {
                match op {
                    b'+' => {
                        result += last_number;
                        last_number = curr;
                    }
                    b'-' => {
                        result += last_number;
                        last_number = -curr;
                    }
                    b'*' => last_number *= curr,
                    _ => last_number /= curr,
                }
                if c == b'+' || c == b'-' || c == b'*' || c == b'/' {
                    op = c;
                }
                curr = 0;
            }
        }
        (result + last_number) as i32
    }
}
