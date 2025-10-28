# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def buildTree(self, inorder: list[int], postorder: list[int]) -> TreeNode | None:
        if not inorder or not postorder:
            return None

        idx = {v: i for i, v in enumerate(inorder)}  # value -> inorder index
        self.pi = len(postorder) - 1                # postorder index (moving leftward)

        def build(lo: int, hi: int) -> TreeNode | None:
            if lo > hi:
                return None
            root_val = postorder[self.pi]
            self.pi -= 1
            mid = idx[root_val]

            # Build right first (because postorder is Left,Right,Root; we're moving from end)
            right = build(mid + 1, hi)
            left = build(lo, mid - 1)

            node = TreeNode(root_val)
            node.left = left
            node.right = right
            return node

        return build(0, len(inorder) - 1)
    