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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        type Node = Option<Rc<RefCell<TreeNode>>>;

        fn inorder(
            node: &Node,
            prev: &mut Node,
            first: &mut Node,
            second: &mut Node,
        ) {
            if let Some(curr_rc) = node {
                // Traverse left
                let left = curr_rc.borrow().left.clone();
                inorder(&left, prev, first, second);

                // Detect inversion with `prev`
                if let Some(p_rc) = prev {
                    let p_val = p_rc.borrow().val;
                    let c_val = curr_rc.borrow().val;
                    if p_val > c_val {
                        if first.is_none() {
                            *first = Some(p_rc.clone());
                        }
                        *second = Some(curr_rc.clone());
                    }
                }
                // Update prev to current
                *prev = Some(curr_rc.clone());

                // Traverse right
                let right = curr_rc.borrow().right.clone();
                inorder(&right, prev, first, second);
            }
        }

        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;

        inorder(root, &mut prev, &mut first, &mut second);

        if let (Some(a), Some(b)) = (first, second) {
            let mut a_b = a.borrow_mut();
            let mut b_b = b.borrow_mut();
            std::mem::swap(&mut a_b.val, &mut b_b.val);
        }
    }
}
