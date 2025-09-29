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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut before_dummy = Box::new(ListNode::new(0));
        let mut after_dummy = Box::new(ListNode::new(0));

        let mut before_tail: &mut Box<ListNode> = &mut before_dummy;
        let mut after_tail: &mut Box<ListNode> = &mut after_dummy;

        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if node.val < x {
                before_tail.next = Some(node);
                before_tail = before_tail.next.as_mut().unwrap();
            } else {
                after_tail.next = Some(node);
                after_tail = after_tail.next.as_mut().unwrap();
            }
        }

        before_tail.next = after_dummy.next.take();
        before_dummy.next
    }
}
