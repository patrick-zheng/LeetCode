use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let pattern: Vec<char> = pattern.chars().collect();
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut char_to_word: HashMap<char, &str> = HashMap::new();
        let mut word_to_char: HashMap<&str, char> = HashMap::new();
        for (&ch, word) in pattern.iter().zip(words.iter()) {
            if let Some(&mapped) = char_to_word.get(&ch) {
                if mapped != *word {
                    return false;
                }
            } else if word_to_char.contains_key(word) {
                return false;
            } else {
                char_to_word.insert(ch, word);
                word_to_char.insert(word, ch);
            }
        }
        true
    }
}
