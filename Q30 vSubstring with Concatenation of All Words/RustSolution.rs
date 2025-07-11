use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let total_len = word_len * words.len();
        let mut result = vec![];

        if s.len() < total_len {
            return result;
        }

        let mut word_count = HashMap::new();
        for word in &words {
            *word_count.entry(word.as_str()).or_insert(0) += 1;
        }

        for i in 0..word_len {
            let mut left = i;
            let mut right = i;
            let mut window_count = HashMap::new();
            let mut count = 0;

            while right + word_len <= s.len() {
                let word = &s[right..right + word_len];
                right += word_len;

                if word_count.contains_key(word) {
                    *window_count.entry(word).or_insert(0) += 1;
                    count += 1;

                    while window_count[word] > word_count[word] {
                        let left_word = &s[left..left + word_len];
                        *window_count.get_mut(left_word).unwrap() -= 1;
                        left += word_len;
                        count -= 1;
                    }

                    if count == words.len() {
                        result.push(left as i32);
                    }
                } else {
                    window_count.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        result
    }
}
