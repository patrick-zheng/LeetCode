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
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut remaining = k;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut cur = root;
        loop {
            while let Some(node) = cur {
                let left = node.borrow().left.clone();
                stack.push(node);
                cur = left;
            }
            let node = stack.pop().expect("non-empty stack when tree has nodes");
            remaining -= 1;
            if remaining == 0 {
                return node.borrow().val;
            }
            cur = node.borrow().right.clone();
        }
    }
}
