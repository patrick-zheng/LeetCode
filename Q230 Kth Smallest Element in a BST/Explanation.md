# LeetCode Problem: Kth Smallest Element in a BST

- **Problem Link**: [Kth Smallest Element in a BST – LeetCode](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/)

---

## Algorithm

In a **binary search tree**, an **in-order** traversal visits node values in
**sorted** order. The \(k\)-th smallest value (1-indexed) is therefore the
\(k\)-th node visited in in-order.

Use an **iterative** in-order walk with an explicit stack (no unbounded
recursion depth):

1. From the current node, repeatedly move **left**, pushing each node onto the
   stack (the next in-order successor is deeper left until `None`).
2. **Pop** the stack: that node is the next smallest in the global order.
   Decrement a `remaining` counter (initialized from `k`; leave `k` itself
   unchanged).
3. When the counter reaches **zero**, return that node's value.
4. Otherwise set the current node to the popped node's **right** child and
   continue (after visiting a node, the subtree to process next is the right
   child).

This uses **only** \(O(h)\) extra space for the stack (height \(h\)); each tree
node is pushed and popped at most once, so time is **linear** in the number of
nodes visited before the answer (worst \(O(n)\) for a skewed tree or large
\(k\)).

**Follow-up:** if the BST changes often and you query the \(k\)-th smallest
repeatedly, augment each node with its left-subtree size (or full subtree size)
so each query descends in **\(O(h)\)** time without scanning in-order.

---

## Constraints

- The number of nodes in the tree is `n`.
- `1 <= k <= n <= 10^4`
- `0 <= Node.val <= 10^4`

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** worst-case — in-order; early exit after k pops (often **O(h + k)**)    |
| Space Complexity   | **O(h)** — stack holds at most one left spine (h = tree height)                 |

<!-- markdownlint-enable MD013 -->

---
