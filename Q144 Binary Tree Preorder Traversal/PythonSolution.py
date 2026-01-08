# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def preorderTraversal(self, root: TreeNode | None) -> list[int]:
        result = []
        
        def traverse(node: TreeNode | None):
            if node is None:
                return
            result.append(node.val)  # Visit the root
            traverse(node.left)      # Traverse left subtree
            traverse(node.right)     # Traverse right subtree
        
        traverse(root)
        return result
