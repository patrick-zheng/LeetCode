from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def swapPairs(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None: return head

        dummy = ListNode()
        dummy.next = head
        curr = dummy

        while curr.next and curr.next.next:
            first = curr.next
            second = first.next

            first.next = second.next
            second.next = first
            curr.next = second

            curr = first

        return dummy.next
