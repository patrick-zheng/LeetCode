pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return true;
        }

        let mut i: usize = 0;
        let mut j: usize = bytes.len() - 1;

        let is_alnum = |c: u8| -> bool {
            (c >= b'0' && c <= b'9') ||
            (c >= b'a' && c <= b'z') ||
            (c >= b'A' && c <= b'Z')
        };

        let to_lower = |c: u8| -> u8 {
            if c >= b'A' && c <= b'Z' {
                c + (b'a' - b'A')
            } else {
                c
            }
        };

        while i < j {
            while i < j && !is_alnum(bytes[i]) {
                i += 1;
            }
            while i < j && !is_alnum(bytes[j]) {
                j -= 1;
            }

            if i >= j {
                break;
            }

            if to_lower(bytes[i]) != to_lower(bytes[j]) {
                return false;
            }

            i += 1;
            j -= 1;
        }

        true
    }
}
