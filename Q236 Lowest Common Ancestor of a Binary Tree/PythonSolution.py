# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode", q: "TreeNode") -> "TreeNode":
        def dfs(node: "TreeNode | None") -> "TreeNode | None":
            if node is None or node is p or node is q:
                return node

            left = dfs(node.left)
            right = dfs(node.right)

            if left and right:
                return node
            return left if left else right

        ans = dfs(root)
        if ans is None:
            raise ValueError("LCA should always exist for valid input")
        return ans
