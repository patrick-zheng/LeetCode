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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
		let mut dummy = Box::new(ListNode { val: 0, next: head });
		let mut fast: *mut Option<Box<ListNode>> = &mut dummy.next;
		let mut slow: *mut Option<Box<ListNode>> = &mut dummy.next;

		// Move `fast` n steps ahead
		for _ in 0..n {
			unsafe {
				if let Some(ref mut node) = *fast {
					fast = &mut node.next;
				}
			}
		}

		// Move both `fast` and `slow` until `fast` hits the end
		unsafe {
			while let Some(ref mut node) = *fast {
				fast = &mut node.next;
				if let Some(ref mut node) = *slow {
					slow = &mut node.next;
				}
			}

			// Remove the nth node
			if let Some(ref mut node) = *slow {
				*slow = node.next.take();
			}
		}

		dummy.next
    }
}

