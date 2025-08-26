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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;

        // 1) Compute length n
        let mut n = 0usize;
        let mut p = head.as_ref();
        while let Some(node) = p {
            n += 1;
            p = node.next.as_ref();
        }
        if n == 0 { return head; }

        let k = (k as usize) % n;
        if k == 0 { return head; }

        // 2) Split at position `split = n - k` (0-based), then append first part to end
        let split = n - k;

        // dummy -> [head], walk `split` steps so that `cur.next` is new_head
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;               // at "index" -1
        for _ in 0..split {
            cur = cur.next.as_mut().unwrap();   // move to next node
        }

        let mut new_head = cur.next.take();     // detach second part: [new_head ... end]
        // find tail of the detached second part
        {
            let mut tail = new_head.as_mut().unwrap();
            while tail.next.is_some() {
                tail = tail.next.as_mut().unwrap();
            }
            // append the first part (dummy.next) after tail
            tail.next = dummy.next.take();      // first part: [original head .. cur]
        }

        new_head
    }
}
