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

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(
            node: Option<Rc<RefCell<TreeNode>>>,
            p_val: i32,
            q_val: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            let current = node?;
            let (val, left, right) = {
                let borrowed = current.borrow();
                (borrowed.val, borrowed.left.clone(), borrowed.right.clone())
            };

            if val == p_val || val == q_val {
                return Some(current);
            }

            let left_found = dfs(left, p_val, q_val);
            let right_found = dfs(right, p_val, q_val);

            if left_found.is_some() && right_found.is_some() {
                return Some(current);
            }
            left_found.or(right_found)
        }

        let p_val = p.as_ref()?.borrow().val;
        let q_val = q.as_ref()?.borrow().val;
        dfs(root, p_val, q_val)
    }
}
