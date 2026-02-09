use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
        val,
        left: None,
        right: None
        }
    }
}

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut it = BSTIterator { stack: Vec::new() };
        it.push_left(root);
        it
    }

    fn push_left(&mut self, mut cur: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(rc) = cur {
            let left = rc.borrow().left.clone();
            self.stack.push(rc);
            cur = left;
        }
    }

    pub fn next(&mut self) -> i32 {
        let rc = self.stack.pop().unwrap();
        let val = rc.borrow().val;

        let right = rc.borrow().right.clone();
        self.push_left(right);

        val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
