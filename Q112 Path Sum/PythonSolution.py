# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def hasPathSum(self, root: TreeNode | None, targetSum: int) -> bool:
        if not root:
            return False
        
        # Check if we are at a leaf node
        if not root.left and not root.right:
            return targetSum == root.val
        
        # Recursively check the left and right subtrees with the updated target sum
        new_target = targetSum - root.val
        return (self.hasPathSum(root.left, new_target) or
                self.hasPathSum(root.right, new_target))
        