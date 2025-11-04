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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, need: i32) -> bool {
            if let Some(rc) = node {
                let n = rc.borrow();
                let remain = need - n.val;
                let is_leaf = n.left.is_none() && n.right.is_none();
                if is_leaf { return remain == 0; }
                dfs(&n.left, remain) || dfs(&n.right, remain)
            } else {
                false
            }
        }
        dfs(&root, target_sum)
    }
}
