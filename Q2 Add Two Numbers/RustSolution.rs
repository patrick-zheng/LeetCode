/*
 * Problem: https://leetcode.com/problems/add-two-numbers/
 * Solution: https://leetcode.com/problems/add-two-numbers/solutions/
 * Time Complexity: O(max(n, m))
 * Space Complexity: O(max(n, m))
 */

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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut answer = ListNode::new(0);
        let mut current = &mut answer;

        let (mut l1, mut l2) = (l1, l2);
        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = carry
                + l1.as_ref().map_or(0, |node| node.val)
                + l2.as_ref().map_or(0, |node| node.val);
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        answer.next
    }
}