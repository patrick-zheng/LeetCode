# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def recoverTree(self, root: TreeNode | None) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        first = second = prev = None
        curr = root

        def detect(p, c):
            nonlocal first, second
            if p and p.val > c.val:
                if not first:
                    first = p
                second = c  # always update to the current node

        while curr:
            if not curr.left:
                detect(prev, curr)
                prev = curr
                curr = curr.right
            else:
                # Find inorder predecessor of curr
                pred = curr.left
                while pred.right and pred.right is not curr:
                    pred = pred.right

                if not pred.right:
                    # Thread: link predecessor to current
                    pred.right = curr
                    curr = curr.left
                else:
                    # Remove thread and visit current
                    pred.right = None
                    detect(prev, curr)
                    prev = curr
                    curr = curr.right

        if first and second:
            first.val, second.val = second.val, first.val
            