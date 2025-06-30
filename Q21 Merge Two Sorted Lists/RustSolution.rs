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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let take_from_list1 = list1.as_ref().unwrap().val < list2.as_ref().unwrap().val;

            let mut node = if take_from_list1 {
                let mut n = list1.take().unwrap();
                list1 = n.next.take();
                n
            } else {
                let mut n = list2.take().unwrap();
                list2 = n.next.take();
                n
            };

            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}

