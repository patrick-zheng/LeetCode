# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def reorderList(self, head: ListNode | None) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        if not head or not head.next or not head.next.next:
            return

        # 1️⃣ Find the middle of the list
        slow = head
        fast = head
        while fast and fast.next:
            slow = slow.next # type: ignore
            fast = fast.next.next

        # 2️⃣ Reverse the second half
        prev = None
        curr = slow.next # type: ignore
        slow.next = None  # type: ignore # split the list

        while curr:
            nxt = curr.next
            curr.next = prev
            prev = curr
            curr = nxt

        # 3️⃣ Merge the two halves
        first = head
        second = prev

        while second:
            tmp1 = first.next # type: ignore
            tmp2 = second.next

            first.next = second # type: ignore
            second.next = tmp1

            first = tmp1
            second = tmp2
            