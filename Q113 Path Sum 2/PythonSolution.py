# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def pathSum(self, root: TreeNode | None, targetSum: int) -> list[list[int]]:
        res = []
        path = []

        def dfs(node: TreeNode | None, remaining: int) -> None:
            if not node:
                return
            path.append(node.val)

            # Check leaf
            if not node.left and not node.right and node.val == remaining:
                res.append(path.copy())

            # Recurse
            dfs(node.left, remaining - node.val)
            dfs(node.right, remaining - node.val)

            # Backtrack
            path.pop()

        dfs(root, targetSum)
        return res
