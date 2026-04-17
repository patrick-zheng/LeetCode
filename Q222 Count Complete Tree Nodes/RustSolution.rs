use std::cell::RefCell;
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
            right: None,
        }
    }
}

fn left_depth(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut d = 0;
    while let Some(n) = node {
        d += 1;
        node = n.borrow().left.clone();
    }
    d
}

pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(rc) = root else {
            return 0;
        };
        let (left_d, right_d, left_child, right_child) = {
            let n = rc.borrow();
            let ld = left_depth(n.left.clone());
            let rd = left_depth(n.right.clone());
            (ld, rd, n.left.clone(), n.right.clone())
        };
        if left_d == right_d {
            (1i32 << left_d) + Self::count_nodes(right_child)
        } else {
            (1i32 << right_d) + Self::count_nodes(left_child)
        }
    }
}
