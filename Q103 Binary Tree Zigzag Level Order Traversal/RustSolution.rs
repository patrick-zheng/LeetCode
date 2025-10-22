use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let Some(r) = root else { return res; };

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(r);

        let mut left_to_right = true;

        while !q.is_empty() {
            let n = q.len();
            let mut level = vec![0; n];

            for i in 0..n {
                let node_rc = q.pop_front().unwrap();
                let node = node_rc.borrow();

                let idx = if left_to_right { i } else { n - 1 - i };
                level[idx] = node.val;

                if let Some(ref l) = node.left {
                    q.push_back(Rc::clone(l));
                }
                if let Some(ref r) = node.right {
                    q.push_back(Rc::clone(r));
                }
            }

            res.push(level);
            left_to_right = !left_to_right;
        }

        res
    }
}
