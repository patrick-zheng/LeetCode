# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def kthSmallest(self, root: TreeNode | None, k: int) -> int:
        stack: list[TreeNode] = []
        cur: TreeNode | None = root
        remaining = k
        while cur or stack:
            while cur:
                stack.append(cur)
                cur = cur.left
            cur = stack.pop()
            remaining -= 1
            if remaining == 0:
                return cur.val
            cur = cur.right
        assert False  # unreachable for valid k and BST
