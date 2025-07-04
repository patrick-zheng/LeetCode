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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut group_prev = dummy.as_mut();

        while let Some(gp) = group_prev {
            // Find the k-th node
            let mut kth = gp.next.as_ref();
            for _ in 0..k-1 {
                if let Some(node) = kth {
                    kth = node.next.as_ref();
                } else {
                    return dummy.unwrap().next;
                }
            }
            if kth.is_none() {
                return dummy.unwrap().next;
            }

            // Reverse k nodes
            let mut prev = kth.unwrap().next.clone();
            let mut curr = gp.next.take();

            for _ in 0..k {
                let mut tmp = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = prev;
                prev = curr;
                curr = tmp;
            }

            gp.next = prev;
            for _ in 0..k {
                group_prev = group_prev.unwrap().next.as_mut();
            }
        }

        dummy.unwrap().next
    }
}
