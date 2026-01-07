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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // Take ownership of the list and collect nodes into a vector
        let mut nodes: Vec<Box<ListNode>> = Vec::new();
        let mut cur = head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
            nodes.push(node);
        }

        if nodes.len() <= 2 {
            // Put it back unchanged
            let mut rebuilt: Option<Box<ListNode>> = None;
            for mut node in nodes.into_iter().rev() {
                node.next = rebuilt;
                rebuilt = Some(node);
            }
            *head = rebuilt;
            return;
        }

        // Re-link nodes in L0, Ln, L1, Ln-1, ...
        let mut i = 0usize;
        let mut j = nodes.len() - 1;

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while i < j {
            // take i
            let mut left = std::mem::replace(&mut nodes[i], Box::new(ListNode::new(0)));
            left.next = None;
            tail.next = Some(left);
            tail = tail.next.as_mut().unwrap();

            // take j
            let mut right = std::mem::replace(&mut nodes[j], Box::new(ListNode::new(0)));
            right.next = None;
            tail.next = Some(right);
            tail = tail.next.as_mut().unwrap();

            i += 1;
            if j == 0 { break; }
            j -= 1;
        }

        // If odd length, attach the middle node
        if i == j {
            let mut mid = std::mem::replace(&mut nodes[i], Box::new(ListNode::new(0)));
            mid.next = None;
            tail.next = Some(mid);
        }

        // Set head to rebuilt list
        *head = dummy.next.take();
    }
}
