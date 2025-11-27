# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def sumNumbers(self, root: TreeNode | None) -> int:
        def dfs(node: TreeNode | None, current: int) -> int:
            if not node:
                return 0
            
            # Build the number for this path
            current = current * 10 + node.val
            
            # If it's a leaf, this path contributes one full number
            if not node.left and not node.right:
                return current
            
            # Otherwise, sum from left and right subtrees
            return dfs(node.left, current) + dfs(node.right, current)
        
        return dfs(root, 0)
    