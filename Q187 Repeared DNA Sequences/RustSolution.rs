use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n < 10 {
            return vec![];
        }

        fn enc(b: u8) -> u32 {
            match b {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                _    => 3, // b'T'
            }
        }

        let mask: u32 = (1u32 << 20) - 1;
        let mut x: u32 = 0;

        let mut seen: HashSet<u32> = HashSet::new();
        let mut added: HashSet<u32> = HashSet::new();
        let mut res: Vec<String> = Vec::new();

        for i in 0..n {
            x = ((x << 2) | enc(bytes[i])) & mask;
            if i >= 9 {
                if seen.contains(&x) {
                    if added.insert(x) {
                        // bytes[i-9..=i] is valid ASCII, so it's valid UTF-8
                        res.push(String::from_utf8(bytes[i - 9..=i].to_vec()).unwrap());
                    }
                } else {
                    seen.insert(x);
                }
            }
        }

        res
    }
}
