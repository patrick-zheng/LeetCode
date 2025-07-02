use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for (i, list) in lists.into_iter().enumerate() {
            if let Some(node) = list {
                heap.push(Reverse((node.val, i, node)));
            }
        }

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let Some(Reverse((_, i, mut node))) = heap.pop() {
            if let Some(next) = node.next.take() {
                heap.push(Reverse((next.val, i, next)));
            }
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

