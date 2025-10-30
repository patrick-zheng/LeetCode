# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sortedListToBST(self, head: ListNode | None) -> TreeNode | None:
        n = 0
        node = head
        while node:
            n += 1
            node = node.next

        curr = head

        def build(k: int):
            nonlocal curr
            if k == 0 or curr is None:
                return None
            # Build left subtree of size k//2
            left = build(k // 2)

            # Root is current list node
            root = TreeNode(curr.val)
            root.left = left

            # Advance list pointer
            curr = curr.next

            # Build right subtree with remaining nodes
            root.right = build(k - 1 - (k // 2))
            return root

        return build(n)