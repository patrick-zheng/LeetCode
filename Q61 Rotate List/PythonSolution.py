from typing import Optional

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if not head or not head.next or k == 0:
            return head

        # 1) find length and tail
        n, tail = 1, head
        while tail.next:
            tail = tail.next
            n += 1

        k %= n
        if k == 0:
            return head

        # 2) make circular
        tail.next = head

        # 3) new tail is (n - k - 1) from head; new head is after it
        new_tail = head
        for _ in range(n - k - 1):
            if new_tail is not None:
                new_tail = new_tail.next  # type: ignore[assignment]

        if new_tail is not None:
            new_head = new_tail.next
            new_tail.next = None  # 4) break the cycle
            return new_head
        else:
            return None
    