# LeetCode Problem: Binary Tree Paths

- **Problem Link**: [Binary Tree Paths – LeetCode](https://leetcode.com/problems/binary-tree-paths/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-paths/solutions/)

---

## Algorithm

Return every **root-to-leaf** path as a string `"a->b->c"`.

### DFS + backtracking

1. If the current node is **null**, return.
2. Push `node.val` onto a working path list.
3. If the node is a **leaf** (no children), join the path with `"->"` and append
   to the answer list.
4. Else recurse on **left** and **right**.
5. **Pop** the last value before returning (backtrack).

Each node is entered and left once, so traversal is linear in the number of
nodes.

---

## Constraints

- The number of nodes in the tree is in the range `[1, 100]`
- `-100 <= Node.val <= 100`

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — visit each node once; build one path string per leaf.                |
| Space Complexity   | **O(h)** — recursion stack and current path; `h` is tree height.                |

<!-- markdownlint-enable MD013 -->

---
