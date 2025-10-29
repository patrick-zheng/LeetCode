# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def levelOrderBottom(self, root: TreeNode | None) -> list[list[int]]:
        from collections import deque
        if not root:
            return []
        q = deque([root])
        levels = []
        while q:
            row = []
            for _ in range(len(q)):
                node = q.popleft()
                row.append(node.val)
                if node.left:  q.append(node.left)
                if node.right: q.append(node.right)
            levels.append(row)
        return levels[::-1]
    