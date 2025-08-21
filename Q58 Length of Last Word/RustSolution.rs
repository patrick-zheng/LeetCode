pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();
        if s.is_empty() {
            return 0;
        }
        let words: Vec<&str> = s.split(' ').collect();
        return words.last().unwrap().len() as i32;
    }
}
