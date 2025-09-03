pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (bytes_a, bytes_b) = (a.as_bytes(), b.as_bytes());
        let (mut i, mut j) = (bytes_a.len() as i32 - 1, bytes_b.len() as i32 - 1);
        let mut carry = 0;
        let mut out: Vec<u8> = Vec::with_capacity(bytes_a.len().max(bytes_b.len()) + 1);

        while i >= 0 || j >= 0 || carry != 0 {
            let bit_a = if i >= 0 { (bytes_a[i as usize] - b'0') as i32 } else { 0 };
            let bit_b = if j >= 0 { (bytes_b[j as usize] - b'0') as i32 } else { 0 };
            let total = bit_a + bit_b + carry;
            out.push(b'0' + (total & 1) as u8);
            carry = total >> 1;
            i -= 1;
            j -= 1;
        }

        out.reverse();
        String::from_utf8(out).unwrap()
    }
}
