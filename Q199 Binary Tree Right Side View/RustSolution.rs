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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, res: &mut Vec<i32>) {
            if let Some(rc_node) = node {
                let node_ref = rc_node.borrow();

                if depth == res.len() {
                    res.push(node_ref.val);
                }

                dfs(node_ref.right.clone(), depth + 1, res);
                dfs(node_ref.left.clone(), depth + 1, res);
            }
        }

        let mut res = Vec::new();
        dfs(root, 0, &mut res);
        res
    }
}
