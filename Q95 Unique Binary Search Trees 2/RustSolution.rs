use std::cell::RefCell;
use std::collections::HashMap;
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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        let mut memo: HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>> = HashMap::new();
        Self::build(1, n, &mut memo)
    }

    fn build(lo: i32, hi: i32, memo: &mut HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if lo > hi {
            return vec![None];
        }
        if let Some(v) = memo.get(&(lo, hi)) {
            return v.clone();
        }

        let mut res = Vec::new();
        for root in lo..=hi {
            let lefts = Self::build(lo, root - 1, memo);
            let rights = Self::build(root + 1, hi, memo);
            for l in &lefts {
                for r in &rights {
                    let node = Rc::new(RefCell::new(TreeNode::new(root)));
                    node.borrow_mut().left = l.clone();
                    node.borrow_mut().right = r.clone();
                    res.push(Some(node));
                }
            }
        }
        memo.insert((lo, hi), res.clone());
        res
    }
}
