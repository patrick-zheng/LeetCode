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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() { return None; }

        // Map value -> index in inorder for O(1) splits
        let mut idx: HashMap<i32, usize> = HashMap::with_capacity(inorder.len());
        for (i, &v) in inorder.iter().enumerate() {
            idx.insert(v, i);
        }

        // Cursor from the end of postorder (root at the end)
        let mut p: isize = postorder.len() as isize - 1;

        fn build(
            lo: isize,
            hi: isize,
            post: &Vec<i32>,
            p: &mut isize,
            idx: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if lo > hi { return None; }

            let root_val = post[*p as usize];
            *p -= 1;

            let mid = *idx.get(&root_val).unwrap() as isize;

            // Build right first, then left (since we consume postorder from the end)
            let right = build(mid + 1, hi, post, p, idx);
            let left  = build(lo, mid - 1, post, p, idx);

            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left,
                right,
            })))
        }

        build(0, inorder.len() as isize - 1, &postorder, &mut p, &idx)
    }
}
