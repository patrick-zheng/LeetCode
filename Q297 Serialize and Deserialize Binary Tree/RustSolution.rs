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

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut tokens = Vec::new();
        Self::dfs_serialize(&root, &mut tokens);
        tokens.join(",")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let tokens: Vec<String> = data.split(',').map(str::to_string).collect();
        let mut index = 0usize;
        Self::dfs_deserialize(&tokens, &mut index)
    }

    fn dfs_serialize(node: &Option<Rc<RefCell<TreeNode>>>, tokens: &mut Vec<String>) {
        match node {
            None => tokens.push("N".to_string()),
            Some(rc) => {
                let borrowed = rc.borrow();
                tokens.push(borrowed.val.to_string());
                Self::dfs_serialize(&borrowed.left, tokens);
                Self::dfs_serialize(&borrowed.right, tokens);
            }
        }
    }

    fn dfs_deserialize(
        tokens: &[String],
        index: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if tokens[*index] == "N" {
            *index += 1;
            return None;
        }
        let val: i32 = tokens[*index].parse().unwrap();
        *index += 1;
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        node.borrow_mut().left = Self::dfs_deserialize(tokens, index);
        node.borrow_mut().right = Self::dfs_deserialize(tokens, index);
        Some(node)
    }
}
