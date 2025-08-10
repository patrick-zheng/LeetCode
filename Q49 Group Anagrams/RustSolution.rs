pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut count = [0; 26];
            for c in word.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }
            map.entry(count).or_default().push(word);
        }

        map.into_values().collect()
    }
}
