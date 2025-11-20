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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(rc_node) => {
                    let n = rc_node.borrow();

                    // Best downward gains from left and right (ignore negatives)
                    let left_gain = dfs(&n.left, max_sum).max(0);
                    let right_gain = dfs(&n.right, max_sum).max(0);

                    // Path that passes through this node
                    let path_through = n.val + left_gain + right_gain;

                    // Update global maximum
                    if path_through > *max_sum {
                        *max_sum = path_through;
                    }

                    // Return best one-side gain to parent
                    n.val + left_gain.max(right_gain)
                }
            }
        }

        let mut max_sum = i32::MIN;
        dfs(&root, &mut max_sum);
        max_sum
    }
}
