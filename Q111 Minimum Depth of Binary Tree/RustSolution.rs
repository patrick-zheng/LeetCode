use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return 0; }
        let mut q: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        q.push_back((root.unwrap(), 1));

        while let Some((node_rc, d)) = q.pop_front() {
            let node = node_rc.borrow();
            let left = node.left.clone();
            let right = node.right.clone();

            if left.is_none() && right.is_none() { return d; }
            if let Some(l) = left { q.push_back((l, d + 1)); }
            if let Some(r) = right { q.push_back((r, d + 1)); }
        }
        0
    }
}
