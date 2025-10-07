# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseBetween(self, head: ListNode | None, left: int, right: int) -> ListNode | None:
        if not head or left == right:
            return head

        dummy = ListNode(0)
        dummy.next = head
        prev = dummy

        # Step 1: move prev to node before 'left'
        for _ in range(left - 1):
            prev = prev.next # type: ignore

        # Step 2: reverse sublist from left to right
        curr = prev.next # type: ignore
        nxt = None
        for _ in range(right - left):
            nxt = curr.next # type: ignore
            curr.next = nxt.next # type: ignore
            nxt.next = prev.next # type: ignore
            prev.next = nxt # type: ignore

        return dummy.next
    