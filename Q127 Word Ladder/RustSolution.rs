use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // If end_word is not in the list, it's impossible to reach
        if !word_list.contains(&end_word) {
            return 0;
        }

        let l = begin_word.len();

        // Build pattern -> list of words mapping
        let mut pattern_map: HashMap<String, Vec<String>> = HashMap::new();

        for word in &word_list {
            for i in 0..l {
                // All words are lowercase English letters → ASCII → byte indices are safe
                let pattern = format!("{}*{}", &word[0..i], &word[i + 1..]);
                pattern_map.entry(pattern).or_default().push(word.clone());
            }
        }

        // BFS
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((begin_word.clone(), 1));

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(begin_word.clone());

        while let Some((current, depth)) = queue.pop_front() {
            for i in 0..l {
                let pattern = format!("{}*{}", &current[0..i], &current[i + 1..]);

                if let Some(neighbors) = pattern_map.get_mut(&pattern) {
                    for neighbor in neighbors.iter() {
                        if neighbor == &end_word {
                            return depth + 1;
                        }

                        if !visited.contains(neighbor) {
                            visited.insert(neighbor.clone());
                            queue.push_back((neighbor.clone(), depth + 1));
                        }
                    }

                    // Clear neighbors to avoid reprocessing the same pattern
                    neighbors.clear();
                }
            }
        }

        0
    }
}
