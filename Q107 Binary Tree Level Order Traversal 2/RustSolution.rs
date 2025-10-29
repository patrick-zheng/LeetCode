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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        if root.is_none() { return res; }

        let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        q.push_back(root.unwrap());

        while !q.is_empty() {
            let mut row: Vec<i32> = Vec::new();
            let level_len = q.len();
            for _ in 0..level_len {
                let node = q.pop_front().unwrap();
                let n = node.borrow();
                row.push(n.val);
                if let Some(l) = &n.left  { q.push_back(Rc::clone(l)); }
                if let Some(r) = &n.right { q.push_back(Rc::clone(r)); }
            }
            res.push(row);
        }
        res.reverse();
        res
    }
}
