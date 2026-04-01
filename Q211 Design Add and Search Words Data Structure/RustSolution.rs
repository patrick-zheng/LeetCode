use std::array;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: array::from_fn(|_| None),
            is_end: false,
        }
    }
}

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            node = node.children[idx]
                .get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(node: &TrieNode, bytes: &[u8], i: usize) -> bool {
            if i == bytes.len() {
                return node.is_end;
            }

            if bytes[i] == b'.' {
                for child in node.children.iter() {
                    if let Some(next) = child.as_ref() {
                        if dfs(next, bytes, i + 1) {
                            return true;
                        }
                    }
                }
                false
            } else {
                let idx = (bytes[i] - b'a') as usize;
                match node.children[idx].as_ref() {
                    Some(next) => dfs(next, bytes, i + 1),
                    None => false,
                }
            }
        }

        dfs(&self.root, word.as_bytes(), 0)
    }
}
