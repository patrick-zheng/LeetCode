# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def maxPathSum(self, root: TreeNode | None) -> int:
        self.max_sum = float("-inf")

        def dfs(node: 'TreeNode') -> int:
            if not node: return 0

            left_gain = max(0, dfs(node.left))
            right_gain = max(0, dfs(node.right))

            path_through = node.val + left_gain + right_gain
            self.max_sum = max(self.max_sum, path_through)
            return node.val + max(left_gain, right_gain)

        dfs(root)
        return self.max_sum