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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if slice.is_empty() { return None; }
            let mid = slice.len() / 2;
            let mut node = TreeNode::new(slice[mid]);
            node.left = build(&slice[..mid]);
            node.right = build(&slice[mid + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
        build(&nums)
    }
}
