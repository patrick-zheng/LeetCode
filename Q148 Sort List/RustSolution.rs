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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Base case
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let (left, right) = Self::split(head);
        let left = Self::sort_list(left);
        let right = Self::sort_list(right);
        Self::merge(left, right)
    }

    // Split list into (left, right) halves
    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        // Count length
        let mut len = 0;
        {
            let mut p = head.as_ref();
            while let Some(node) = p {
                len += 1;
                p = node.next.as_ref();
            }
        }

        // Walk to the end of left half: (len/2 - 1) steps
        let mid = len / 2;
        let mut cur = head.as_mut();
        for _ in 0..(mid - 1) {
            cur = cur.unwrap().next.as_mut();
        }

        // Detach right half
        let right = cur.as_mut().unwrap().next.take();
        (head, right)
    }

    // Merge two sorted lists
    fn merge(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while a.is_some() && b.is_some() {
            let take_a = a.as_ref().unwrap().val < b.as_ref().unwrap().val;

            if take_a {
                let next = a.as_mut().unwrap().next.take();
                tail.next = a.take();
                tail = tail.next.as_mut().unwrap();
                a = next;
            } else {
                let next = b.as_mut().unwrap().next.take();
                tail.next = b.take();
                tail = tail.next.as_mut().unwrap();
                b = next;
            }
        }

        tail.next = if a.is_some() { a } else { b };
        dummy.next
    }
}
