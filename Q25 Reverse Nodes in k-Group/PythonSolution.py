from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        dummy = ListNode()
        dummy.next = head
        group_prev = dummy

        while True:
            kth = group_prev
            count = 0
            while count < k and kth:
                kth = kth.next
                count += 1
            if not kth: break

            prev, curr = kth.next, group_prev.next

            for _ in range(k):
                tmp = curr.next
                curr.next = prev
                prev = curr
                curr = tmp

            tmp = group_prev.next
            group_prev.next = kth
            group_prev = tmp

        return dummy.next
