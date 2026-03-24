pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut s_to_t = [-1i32; 256];
        let mut t_to_s = [-1i32; 256];

        for i in 0..s_bytes.len() {
            let cs = s_bytes[i] as usize;
            let ct = t_bytes[i] as usize;

            if s_to_t[cs] == -1 && t_to_s[ct] == -1 {
                s_to_t[cs] = ct as i32;
                t_to_s[ct] = cs as i32;
            } else if s_to_t[cs] != ct as i32 || t_to_s[ct] != cs as i32 {
                return false;
            }
        }

        true
    }
}
