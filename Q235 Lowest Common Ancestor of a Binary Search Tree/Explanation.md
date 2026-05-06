# LeetCode Problem: Lowest Common Ancestor of a Binary Search Tree

- **Problem Link**: [Lowest Common Ancestor of a Binary Search Tree - LeetCode](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/solutions/)

---

## Algorithm

Because this is a **binary search tree (BST)**:

- If both `p` and `q` are smaller than `root`, their LCA must be in the left
  subtree.
- If both are greater than `root`, their LCA must be in the right subtree.
- Otherwise, the current node is the split point (or equals one target), so it
  is the LCA.

We iterate from the root and move left/right until we find that split point.

### Steps

1. Start from `cur = root`.
2. If `p.val` and `q.val` are both `< cur.val`, set `cur = cur.left`.
3. Else if `p.val` and `q.val` are both `> cur.val`, set `cur = cur.right`.
4. Otherwise return `cur` (this is the lowest common ancestor).

This is optimal because we only follow one root-to-node path, never exploring
both subtrees.

---

## Constraints

- The number of nodes in the tree is in the range `[2, 10^5]`.
- `-10^9 <= Node.val <= 10^9`
- All `Node.val` are unique.
- `p != q`
- `p` and `q` will exist in the BST.

---

## Complexity

| Metric           | Complexity                             |
|------------------|----------------------------------------|
| Time Complexity  | **O(h)**, where `h` is the BST height. |
| Space Complexity | **O(1)** iterative traversal.          |

---
