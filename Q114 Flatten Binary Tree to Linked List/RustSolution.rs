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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        if let Some(r) = root.clone() {
            stack.push(r);
        }

        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while let Some(node_rc) = stack.pop() {
            if let Some(ref p) = prev {
                // Link previous to current in "list" form
                let mut pm = p.borrow_mut();
                pm.left = None;
                pm.right = Some(node_rc.clone());
            }

            // Push right first, then left, to process left next (preorder)
            {
                let nm = node_rc.borrow();
                if let Some(r) = nm.right.clone() {
                    stack.push(r);
                }
                if let Some(l) = nm.left.clone() {
                    stack.push(l);
                }
            }

            prev = Some(node_rc);
        }
    }
}
