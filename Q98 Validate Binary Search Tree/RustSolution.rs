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

pub struct Solution {}
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, low: i64, high: i64) -> bool {
            if let Some(rc) = node {
                let n = rc.borrow();
                let v = n.val as i64;
                if v <= low || v >= high {
                    return false;
                }
                dfs(&n.left, low, v) && dfs(&n.right, v, high)
            } else {
                true
            }
        }
        dfs(&root, i64::MIN, i64::MAX)
    }
}
