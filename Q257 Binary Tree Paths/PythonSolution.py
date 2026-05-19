# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def binaryTreePaths(self, root: TreeNode | None) -> list[str]:
        out: list[str] = []

        def dfs(node: TreeNode | None, path: list[int]) -> None:
            if node is None:
                return
            path.append(node.val)
            if node.left is None and node.right is None:
                out.append("->".join(str(v) for v in path))
            else:
                dfs(node.left, path)
                dfs(node.right, path)
            path.pop()

        dfs(root, [])
        return out
