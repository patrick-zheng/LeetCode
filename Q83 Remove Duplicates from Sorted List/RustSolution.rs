// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
        let mut head = head;
        let mut cur = head.as_mut();

        while let Some(node) = cur {
            // Remove all consecutive duplicates of node.val
            while let Some(next) = node.next.as_mut() {
                if next.val == node.val {
                    // Skip the duplicate by linking to next.next
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            cur = node.next.as_mut();
        }

        head
    }
}
