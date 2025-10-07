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
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        if left == right { return head; }

        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut p = &mut dummy;

        for _ in 0..(left - 1) { p = &mut p.as_mut().unwrap().next; }

        let mut curr = p.as_mut().unwrap().next.take();
        let mut prev_sub: Option<Box<ListNode>> = None;

        for _ in left..=right {
            if let Some(mut node) = curr {
                let next = node.next.take(); // detach
                node.next = prev_sub;        // push-front into reversed
                prev_sub = Some(node);
                curr = next;                 // advance
            }
        }

        {
            let mut tail_ref = prev_sub.as_mut().unwrap();
            while tail_ref.next.is_some() {
                tail_ref = tail_ref.next.as_mut().unwrap();
            }
            tail_ref.next = curr; // connect tail -> rest
        }

        p.as_mut().unwrap().next = prev_sub;
        dummy.unwrap().next
    }
}
