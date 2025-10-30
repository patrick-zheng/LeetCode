use std::rc::Rc;
use std::cell::RefCell;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

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
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut head = head;

        // Count length n.
        let n = {
            let mut cnt = 0;
            let mut p = head.as_ref();
            while let Some(node) = p {
                cnt += 1;
                p = node.next.as_ref();
            }
            cnt
        };

        // Build a balanced BST from the next k nodes, advancing `head` as we go.
        fn build(
            k: i32,
            head: &mut Option<Box<ListNode>>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if k == 0 {
                return None;
            }

            // Left subtree: k/2 nodes.
            let left = build(k / 2, head);

            // Current root from the list head (must exist if counts are correct).
            let mut node = head.take().expect("list shorter than expected");
            let val = node.val;
            *head = node.next.take(); // advance exactly once

            let root = Rc::new(RefCell::new(TreeNode::new(val)));

            // Right subtree: remaining nodes.
            let right = build(k - 1 - (k / 2), head);

            {
                let mut r = root.borrow_mut();
                r.left = left;
                r.right = right;
            }

            Some(root)
        }

        build(n as i32, &mut head)
    }
}
