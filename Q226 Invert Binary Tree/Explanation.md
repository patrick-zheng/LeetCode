# LeetCode Problem: Invert Binary Tree

- **Problem Link**: [Invert Binary Tree – LeetCode](https://leetcode.com/problems/invert-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/invert-binary-tree/solutions/)

---

## Algorithm

Mirroring the tree means every node’s **left** and **right** children are **swapped**,
and the same rule applies recursively to both subtrees.

**DFS (post-order style on pointers):**

1. If the current node is **null**, return **null**.
2. **Swap** `left` and `right` child pointers (or swap the two subtree roots).
3. Recursively **invert** the new left subtree and the new right subtree.
4. Return the current node as the root of the inverted subtree.

Each edge of the tree is followed a constant number of times, so the work is
linear in the number of nodes.

An equivalent **iterative BFS** version uses a queue: dequeue a node, swap its
children, enqueue non-null children. Same **O(n)** time; extra space is
**O(w)** for the queue width **w** (up to **O(n)**).

---

## Constraints

- The number of nodes in the tree is in the range `[0, 100]`
- `-100 <= Node.val <= 100`

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one visit per node; swap left and right child pointers at each node  |
| Space Complexity   | **O(h)** — recursion stack depth `h`; worst **O(n)** if tree is a path          |

<!-- markdownlint-enable MD013 -->

---
