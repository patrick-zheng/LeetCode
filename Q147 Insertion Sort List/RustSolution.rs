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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut node) = curr {
            curr = node.next.take(); // detach node from original list

            // Insert node into sorted list
            if sorted.is_none() || sorted.as_ref().unwrap().val >= node.val {
                node.next = sorted;
                sorted = Some(node);
            } else {
                let mut p = sorted.as_mut().unwrap();
                while p.next.is_some() && p.next.as_ref().unwrap().val < node.val {
                    p = p.next.as_mut().unwrap();
                }
                node.next = p.next.take();
                p.next = Some(node);
            }
        }

        sorted
    }
}
