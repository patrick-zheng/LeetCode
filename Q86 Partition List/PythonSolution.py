# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def partition(self, head: ListNode | None, x: int) -> ListNode | None:
        before_dummy = ListNode(0)
        after_dummy = ListNode(0)
        before_tail, after_tail = before_dummy, after_dummy

        curr = head
        while curr:
            nxt = curr.next
            curr.next = None
            if curr.val < x:
                before_tail.next = curr
                before_tail = curr
            else:
                after_tail.next = curr
                after_tail = curr
            curr = nxt

        before_tail.next = after_dummy.next
        return before_dummy.next
