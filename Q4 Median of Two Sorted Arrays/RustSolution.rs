pub struct Solution;
impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_index_map = std::collections::HashMap::new();
    let mut max_length = 0;
    let mut start = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_index_map.get(&c) {
            start = start.max(prev_index + 1);
        }
        char_index_map.insert(c, i);
        max_length = max_length.max(i - start + 1);
    }

    max_length as i32
  }
}