use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }

        let mut res = String::new();

        // sign
        if (numerator < 0) ^ (denominator < 0) {
            res.push('-');
        }

        let mut n = (numerator as i64).abs();
        let d = (denominator as i64).abs();

        // integer part
        res.push_str(&(n / d).to_string());
        let mut rem = n % d;
        if rem == 0 {
            return res;
        }

        res.push('.');

        // remainder -> index in res where the digit for this remainder starts
        let mut seen: HashMap<i64, usize> = HashMap::new();

        while rem != 0 {
            if let Some(&idx) = seen.get(&rem) {
                res.insert(idx, '(');
                res.push(')');
                break;
            }

            seen.insert(rem, res.len());

            rem *= 10;
            let digit = (rem / d) as u8;
            res.push((b'0' + digit) as char);
            rem %= d;
        }

        res
    }
}
