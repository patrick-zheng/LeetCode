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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(
            a: Option<Rc<RefCell<TreeNode>>>,
            b: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (a, b) {
                (None, None) => true,
                (Some(la), Some(rb)) => {
                    let la_b = la.borrow();
                    let rb_b = rb.borrow();
                    la_b.val == rb_b.val
                        && is_mirror(la_b.left.clone(), rb_b.right.clone())
                        && is_mirror(la_b.right.clone(), rb_b.left.clone())
                }
                _ => false,
            }
        }
        is_mirror(root.clone(), root)
    }
}
