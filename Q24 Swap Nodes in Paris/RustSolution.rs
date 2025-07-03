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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut first) => {
                if let Some(mut second) = first.next.take() {
                    let rest = Self::swap_pairs(second.next.take());
                    second.next = Some(first);
                    second.next.as_mut().unwrap().next = rest;
                    Some(second)
                } else {
                    Some(first)
                }
            }
            None => None,
        }
    }
}

