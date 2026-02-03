pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n = column_number;   // shadow with mutable variable
        let mut chars: Vec<u8> = Vec::new();

        while n > 0 {
            n -= 1;                  // shift to 0-based
            let rem = (n % 26) as u8;
            chars.push(b'A' + rem);
            n /= 26;
        }

        chars.reverse();
        String::from_utf8(chars).unwrap()
    }
}
