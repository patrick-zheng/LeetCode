pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let b = s.as_bytes();
        if b.is_empty() || b[0] == b'0' { return 0; }

        let (mut prev2, mut prev1) = (1_i64, 1_i64);
        for i in 1..b.len() {
            let mut curr = 0_i64;
            if b[i] != b'0' { curr += prev1; }
            let two = (b[i-1] - b'0') as i32 * 10 + (b[i] - b'0') as i32;
            if (10..=26).contains(&two) { curr += prev2; }
            if curr == 0 { return 0; }
            prev2 = prev1;
            prev1 = curr;
        }
        prev1 as i32
    }
}
