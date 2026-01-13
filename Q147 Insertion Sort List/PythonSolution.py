# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def insertionSortList(self, head: ListNode | None) -> ListNode | None:
        if not head:
            return head
        
        dummy = ListNode(0)
        dummy.next = head
        last_sorted = head
        curr = head.next
        
        while curr:
            if last_sorted.val <= curr.val:
                last_sorted = last_sorted.next
            else:
                prev = dummy
                while prev.next.val <= curr.val:
                    prev = prev.next
                
                last_sorted.next = curr.next
                curr.next = prev.next
                prev.next = curr
            
            curr = last_sorted.next
        
        return dummy.next
