use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let Some(start) = root else { return res };
        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(start);

        while !q.is_empty() {
            let k = q.len();
            let mut level = Vec::with_capacity(k);
            for _ in 0..k {
                if let Some(node_rc) = q.pop_front() {
                    let node = node_rc.borrow();
                    level.push(node.val);
                    if let Some(l) = node.left.clone()  { q.push_back(l); }
                    if let Some(r) = node.right.clone() { q.push_back(r); }
                }
            }
            res.push(level);
        }
        res
    }
}
