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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let (a_b, b_b) = (a.borrow(), b.borrow());
                a_b.val == b_b.val
                    && Self::is_same_tree(a_b.left.clone(),  b_b.left.clone())
                    && Self::is_same_tree(a_b.right.clone(), b_b.right.clone())
            }
            _ => false,
        }
    }
}
