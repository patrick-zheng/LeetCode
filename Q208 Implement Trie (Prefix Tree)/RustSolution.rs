use std::cell::RefCell;

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: std::array::from_fn(|_| None),
            is_end: false,
        }
    }
}

struct Trie {
    root: RefCell<TrieNode>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            root: RefCell::new(TrieNode::new()),
        }
    }
    
    fn insert(&self, word: String) {
        let mut node = self.root.borrow_mut();
        let mut curr: &mut TrieNode = &mut node;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            curr = curr.children[idx]
                .get_or_insert_with(|| Box::new(TrieNode::new()))
                .as_mut();
        }

        curr.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let node = self.root.borrow();
        let mut curr: &TrieNode = &node;

        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            match curr.children[idx].as_ref() {
                Some(next) => curr = next.as_ref(),
                None => return false,
            }
        }

        curr.is_end
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let node = self.root.borrow();
        let mut curr: &TrieNode = &node;

        for b in prefix.bytes() {
            let idx = (b - b'a') as usize;
            match curr.children[idx].as_ref() {
                Some(next) => curr = next.as_ref(),
                None => return false,
            }
        }

        true
    }
}
