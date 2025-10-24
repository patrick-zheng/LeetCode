use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() { return None; }

        let mut in_pos: HashMap<i32, usize> = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            in_pos.insert(v, i);
        }

        let mut pre_idx: usize = 0;

        fn build(
            preorder: &Vec<i32>,
            lo: isize,
            hi: isize,
            pre_idx: &mut usize,
            in_pos: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if lo > hi { return None; }

            let val = preorder[*pre_idx];
            *pre_idx += 1;

            let mid = *in_pos.get(&val).unwrap() as isize;

            let left  = build(preorder, lo, mid - 1, pre_idx, in_pos);
            let right = build(preorder, mid + 1, hi, pre_idx, in_pos);

            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }

        build(&preorder, 0, inorder.len() as isize - 1, &mut pre_idx, &in_pos)
    }
}
