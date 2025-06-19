"""
Problem: https://leetcode.com/problems/add-two-numbers/
Solution: https://leetcode.com/problems/add-two-numbers/solutions/
Time Complexity: O(max(n, m))
Space Complexity: O(max(n, m))
"""

# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        answer: ListNode = ListNode()
        current: ListNode = answer
        carry: int = 0

        while l1 or l2 or carry:
            l1_value = l1.val if l1 else 0
            l2_value = l2.val if l2 else 0

            total: int = l1_value + l2_value + carry
            carry: int = total // 10
            current.next = ListNode(total % 10)
            current = current.next

            l1 = l1.next if l1 else None
            l2 = l2.next if l2 else None

        return answer.next

