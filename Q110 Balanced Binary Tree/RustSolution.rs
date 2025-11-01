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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
         fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(rc) => {
                    let n = rc.borrow();
                    let lh = height(&n.left);
                    if lh == -1 { return -1; }
                    let rh = height(&n.right);
                    if rh == -1 { return -1; }
                    if (lh - rh).abs() > 1 { return -1; }
                    1 + lh.max(rh)
                }
            }
        }
        height(&root) != -1
    }
}
