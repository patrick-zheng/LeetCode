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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut dummy;

        while let Some(mut cur) = prev.next.take() {
            let val = cur.val;
            let mut dup = false;

            loop {
                match cur.next.as_ref() {
                    Some(next) if next.val == val => {
                        dup = true;
                        let mut nxt = cur.next.take().unwrap();
                        cur.next = nxt.next.take();
                    }
                    _ => break,
                }
            }

            if dup {
                prev.next = cur.next.take();
            } else {
                prev.next = Some(cur);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}
