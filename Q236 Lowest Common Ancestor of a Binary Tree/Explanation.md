# LeetCode Problem: Lowest Common Ancestor of a Binary Tree

- **Problem Link**: [Lowest Common Ancestor of a Binary Tree - LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/)

---

## Algorithm

For a general binary tree (not necessarily a BST), the standard optimal
approach is a postorder DFS.

### Key idea

For each node:

- If it is `None`, it contributes no target.
- If it equals `p` or `q`, return that node upward.
- Recursively ask left and right children for results.
  - If both sides return non-null, current node is the first split point, so it
    is the LCA.
  - Otherwise return whichever side is non-null.

This guarantees the *lowest* common ancestor because the first node where `p`
and `q` appear in different branches during bottom-up traversal is exactly the
lowest shared ancestor.

### Steps

1. Run DFS from `root`.
2. Base case: return `node` when `node is None`, `node is p`, or `node is q`.
3. Compute `left = dfs(node.left)` and `right = dfs(node.right)`.
4. If `left` and `right` are both non-null, return `node`.
5. Else return `left` if present, otherwise `right`.

---

## Constraints

- The number of nodes in the tree is in the range `[2, 10^5]`.
- `-10^9 <= Node.val <= 10^9`
- All `Node.val` are unique.
- `p != q`
- `p` and `q` will exist in the tree.

---

## Complexity

| Metric           | Complexity                                  |
|------------------|---------------------------------------------|
| Time Complexity  | **O(n)**, visiting each node once.          |
| Space Complexity | **O(h)** stack; worst `O(n)`.               |

---
