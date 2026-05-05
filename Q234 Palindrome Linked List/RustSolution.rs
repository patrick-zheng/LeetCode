// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = next;
        }
        prev
    }

    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let mut len = 0usize;
        let mut walker = head.as_ref();
        while let Some(node) = walker {
            len += 1;
            walker = node.next.as_ref();
        }

        if len <= 1 {
            return true;
        }

        let mut split = &mut head;
        for _ in 0..(len / 2) {
            split = &mut split.as_mut().unwrap().next;
        }
        if len % 2 == 1 {
            split = &mut split.as_mut().unwrap().next;
        }

        let second_half = split.take();
        let reversed_second = Self::reverse_list(second_half);

        let mut left = head.as_ref();
        let mut right = reversed_second.as_ref();
        for _ in 0..(len / 2) {
            if left.unwrap().val != right.unwrap().val {
                return false;
            }
            left = left.unwrap().next.as_ref();
            right = right.unwrap().next.as_ref();
        }

        true
    }
}
