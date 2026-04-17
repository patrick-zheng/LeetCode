# LeetCode Problem: Count Complete Tree Nodes

- **Problem Link**: [Count Complete Tree Nodes – LeetCode](https://leetcode.com/problems/count-complete-tree-nodes/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/count-complete-tree-nodes/solutions/)

---

## Algorithm

We need the number of nodes in a **complete** binary tree without visiting every
node in **O(n)** if we can do better.

Let **`d(node)`** be the number of nodes on the path that starts at `node` and
repeatedly follows **`node.left`** until `None` (a “left spine” length).

At root `r`:

1. Compute **`a = d(r.left)`** and **`b = d(r.right)`** (each **O(h)** with tree
   height **`h`**).
2. If **`a == b`**, the **left** subtree is a **perfect** binary tree: it has
   exactly **`2^a - 1`** nodes, and together with the root the block
   **`2^a`** nodes are accounted for. The remainder is **`countNodes(r.right)`**.
3. Otherwise **`a > b`** (complete tree shape), the **right** subtree is
   perfect with **`2^b - 1`** nodes plus the root gives **`2^b`** accounted; the
   remainder is **`countNodes(r.left)`**.

Recurrence:

\[
\text{count}(r) =
\begin{cases}
0 & r = \text{None} \\
2^{a} + \text{count}(r.\text{right}) & a = b \\
2^{b} + \text{count}(r.\text{left}) & a \neq b
\end{cases}
\]

Each level removes half the unknown height in the worst case, and each step
does two spine walks of length **O(h)**, so time is **O(h²) = O((\log n)²)**.

---

## Constraints

- `0 <= number of nodes <= 5 * 10^4`
- `0 <= Node.val <= 5 * 10^4`
- The tree is **complete** (all levels full except possibly the last, filled
  left to right).

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(h²)** — two left-spine walks per call; recursion depth `O(h)` (`h = log n`).|
| Space Complexity   | **O(h)** — recursion stack only; tree height `h` is `O(log n)`.                 |

<!-- markdownlint-enable MD013 -->

---
