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

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut curr = root;

        while curr.is_some() || !stack.is_empty() {
            // Go as left as possible
            while let Some(node) = curr {
                curr = node.borrow().left.clone();
                stack.push(node);
            }

            // Pop and process node
            if let Some(node) = stack.pop() {
                res.push(node.borrow().val);
                curr = node.borrow().right.clone();
            }
        }
        res
    }
}
