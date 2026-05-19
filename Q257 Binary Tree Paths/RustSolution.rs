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

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut out = Vec::new();
        Self::dfs(&root, &mut Vec::new(), &mut out);
        out
    }

    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<i32>,
        out: &mut Vec<String>,
    ) {
        let Some(n) = node else {
            return;
        };
        let n = n.borrow();
        path.push(n.val);
        if n.left.is_none() && n.right.is_none() {
            out.push(
                path.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("->"),
            );
        } else {
            Self::dfs(&n.left, path, out);
            Self::dfs(&n.right, path, out);
        }
        path.pop();
    }
}
