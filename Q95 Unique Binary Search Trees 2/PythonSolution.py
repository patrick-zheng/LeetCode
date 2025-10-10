from functools import lru_cache

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def generateTrees(self, n: int) -> list[TreeNode | None]:
        if n == 0:
            return []

        @lru_cache(None)
        def build(lo: int, hi: int) -> tuple[TreeNode | None, ...]:
            # Return all unique BSTs that use values in [lo, hi]
            if lo > hi:
                return (None,)  # empty tree as a valid subtree

            all_trees = []
            for root_val in range(lo, hi + 1):
                left_subtrees  = build(lo, root_val - 1)
                right_subtrees = build(root_val + 1, hi)
                for L in left_subtrees:
                    for R in right_subtrees:
                        root = TreeNode(root_val, L, R)
                        all_trees.append(root)
            # Cache as tuple (hashable); caller converts to list when needed
            return tuple(all_trees)

        return list(build(1, n))
