use std::rc::Rc;
use std::cell::RefCell;


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

pub struct Solution;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();

        if let Some(node) = root {
            stack.push(node);
        }

        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            result.push(node_ref.val);

            if let Some(right) = node_ref.right.clone() {
                stack.push(right);
            }
            if let Some(left) = node_ref.left.clone() {
                stack.push(left);
            }
        }

        result
    }
}
