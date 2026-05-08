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
    pub fn delete_node(node: &mut ListNode) {
        let mut next = node.next.take().expect("node must not be tail");
        node.val = next.val;
        node.next = next.next.take();
    }
}
