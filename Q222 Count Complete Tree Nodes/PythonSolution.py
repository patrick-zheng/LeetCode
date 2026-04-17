# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def countNodes(self, root: TreeNode | None) -> int:
        if not root:
            return 0
        left_depth = self._left_depth(root.left)
        right_depth = self._left_depth(root.right)
        if left_depth == right_depth:
            return (1 << left_depth) + self.countNodes(root.right)
        return (1 << right_depth) + self.countNodes(root.left)

    def _left_depth(self, node: TreeNode | None) -> int:
        d = 0
        while node:
            d += 1
            node = node.left
        return d
