pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let first = strs[0].as_bytes();
        for i in 0..first.len() {
            let c = first[i];
            for s in strs.iter().skip(1) {
                let bytes = s.as_bytes();
                if i >= bytes.len() || bytes[i] != c {
                    return strs[0][..i].to_string();
                }
            }
        }
        strs[0].clone()
    }
}
