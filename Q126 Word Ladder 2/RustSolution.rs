use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut word_set: HashSet<String> = word_list.into_iter().collect();
        let mut res: Vec<Vec<String>> = Vec::new();

        // If end_word not in dictionary, no solution
        if !word_set.contains(&end_word) {
            return res;
        }

        // dist[word] = shortest distance from begin_word
        let mut dist: HashMap<String, i32> = HashMap::new();
        // parents[word] = list of previous words on any shortest path to word
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();

        dist.insert(begin_word.clone(), 0);
        let mut q: VecDeque<String> = VecDeque::new();
        q.push_back(begin_word.clone());

        let word_len = begin_word.len();

        // ---------- BFS to fill dist and parents ----------
        while !q.is_empty() {
            let level_size = q.len();
            let mut level_visited: HashSet<String> = HashSet::new();

            for _ in 0..level_size {
                let cur = q.pop_front().unwrap();
                let cur_dist = *dist.get(&cur).unwrap();

                // Turn current word into char vector to modify letters
                let mut chars: Vec<char> = cur.chars().collect();

                for i in 0..word_len {
                    let original = chars[i];

                    for b in b'a'..=b'z' {
                        let ch = b as char;
                        if ch == original {
                            continue;
                        }
                        chars[i] = ch;
                        let next: String = chars.iter().collect();

                        if !word_set.contains(&next) {
                            continue;
                        }

                        // First time we reach `next`
                        if !dist.contains_key(&next) {
                            dist.insert(next.clone(), cur_dist + 1);
                            parents
                                .entry(next.clone())
                                .or_insert_with(Vec::new)
                                .push(cur.clone());
                            q.push_back(next.clone());
                            level_visited.insert(next);
                        }
                        // Another shortest path to `next`
                        else if dist.get(&next) == Some(&(cur_dist + 1)) {
                            parents
                                .entry(next.clone())
                                .or_insert_with(Vec::new)
                                .push(cur.clone());
                        }
                    }

                    // Restore original character
                    chars[i] = original;
                }
            }

            // Remove all words visited at this level so we don't expand them again
            for w in level_visited {
                word_set.remove(&w);
            }
        }

        // If end_word unreachable, return empty
        if !dist.contains_key(&end_word) {
            return res;
        }

        // ---------- DFS / Backtracking from end_word to begin_word ----------
        let mut path: Vec<String> = Vec::new();
        path.push(end_word.clone());

        fn dfs(
            word: &String,
            begin_word: &String,
            parents: &HashMap<String, Vec<String>>,
            path: &mut Vec<String>,
            res: &mut Vec<Vec<String>>,
        ) {
            if word == begin_word {
                let mut seq = path.clone();
                seq.reverse();
                res.push(seq);
                return;
            }

            if let Some(ps) = parents.get(word) {
                for p in ps {
                    path.push(p.clone());
                    dfs(p, begin_word, parents, path, res);
                    path.pop();
                }
            }
        }

        dfs(&end_word, &begin_word, &parents, &mut path, &mut res);
        res
    }
}
