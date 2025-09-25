# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
        
class Solution:
    def deleteDuplicates(self, head: ListNode | None) -> ListNode | None:
        dummy = ListNode(0, head)
        prev = dummy
        cur = head

        while cur:
            run_val = cur.val
            run_len = 0
            # Count the run length and stop at first different value
            while cur and cur.val == run_val:
                cur = cur.next
                run_len += 1

            if run_len == 1:
                # prev.next currently points to the single distinct node (head of run)
                prev = prev.next # type: ignore
            else:
                # skip entire run
                prev.next = cur

        return dummy.next
    