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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
         fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, current: i32) -> i32 {
            match node {
                None => 0,
                Some(rc_node) => {
                    let n = rc_node.borrow();
                    let cur = current * 10 + n.val;

                    // Leaf node
                    if n.left.is_none() && n.right.is_none() {
                        cur
                    } else {
                        dfs(&n.left, cur) + dfs(&n.right, cur)
                    }
                }
            }
        }

        dfs(&root, 0)
    }
}
