# Definition for singly-linked list.
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

class Solution:
    def detectCycle(self, head: ListNode | None) -> ListNode | None:
        if not head:
            return None

        slow = head
        fast = head

        # Step 1: Determine if there is a cycle
        while fast and fast.next:
            slow = slow.next # type: ignore
            fast = fast.next.next

            if slow == fast:
                break
        else:
            return None  # No cycle

        # Step 2: Find the entry point of the cycle
        slow = head
        while slow != fast:
            slow = slow.next # type: ignore
            fast = fast.next

        return slow
