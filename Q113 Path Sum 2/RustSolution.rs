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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, rem: i32, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
            if let Some(rc) = node {
                let n = rc.borrow();
                path.push(n.val);
                if n.left.is_none() && n.right.is_none() && n.val == rem {
                    out.push(path.clone());
                }
                dfs(n.left.clone(), rem - n.val, path, out);
                dfs(n.right.clone(), rem - n.val, path, out);
                path.pop();
            }
        }

        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(root, target_sum, &mut path, &mut out);
        out
    }
}
