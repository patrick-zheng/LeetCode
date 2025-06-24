pub struct Solution;

use std::i32;

impl Solution {
    pub fn functionName() -> void {
        let bytes = s.as_bytes();
        let mut i = 0;
        let n = bytes.len();
        let mut sign: i32 = 1;
        let mut result: i32 = 0;

        // Skip leading whitespace
        while i < n && bytes[i] == b' ' {
            i += 1;
        }

        // Handle optional sign
        if i < n {
            if bytes[i] == b'-' {
                sign = -1;
                i += 1;
            } else if bytes[i] == b'+' {
                i += 1;
            }
        }

        // Process digits and handle overflow
        while i < n && bytes[i].is_ascii_digit() {
            let digit = (bytes[i] - b'0') as i32;

            // Check for overflow before it happens
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > i32::MAX % 10) {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            result = result * 10 + digit;
            i += 1;
        }

        result * sign
    }
}
