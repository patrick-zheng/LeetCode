# Definition for a binary tree node.
class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def lowestCommonAncestor(self, root: "TreeNode", p: "TreeNode", q: "TreeNode") -> "TreeNode":
        cur = root
        p_val = p.val
        q_val = q.val
        while cur:
            if p_val < cur.val and q_val < cur.val:
                cur = cur.left
            elif p_val > cur.val and q_val > cur.val:
                cur = cur.right
            else:
                return cur
        raise ValueError("LCA should always exist for valid input")
