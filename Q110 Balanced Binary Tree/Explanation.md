# 🧩 LeetCode Problem: Balanced Binary Tree (LC 110)

- **Problem Link**: [Balanced Binary Tree – LeetCode](https://leetcode.com/problems/balanced-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/balanced-binary-tree/solutions/)

---

## 🧠 Algorithm Explanation

A binary tree is **height-balanced** if, at **every** node, the heights of the left and right subtrees differ by at most 1.

Use a **post-order DFS** that returns the height of a subtree. If a subtree is unbalanced, return a sentinel value (e.g., `-1`) immediately and propagate it upward (early exit). Otherwise, return `max(left_height, right_height) + 1`. The root is balanced iff the final return is not `-1`.

***Why this works***

- Post-order ensures both child heights are known before judging the current node.
- The sentinel avoids extra recomputation and stops as soon as an imbalance is found, yielding true `O(n)` time.

---

### 🪜 Steps

1. **Recurse left & right** to compute their heights.
2. **Check imbalance**: if either child returned `-1` or `|lh − rh| > 1`, return `-1`.
3. **Return height**: otherwise return `max(lh, rh) + 1`.
4. **Answer**: tree is balanced iff the root call’s result ≠ `-1`.

---

## ✅ Constraints

- The tree can be **empty** (which is balanced).
- Node values don’t matter for balance—only structure/height does.
- “Balanced” means `|height(left) − height(right)| ≤ 1` at **every** node.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                    |
|-------------------|-------------------------------|
| Time Complexity   | `O(n)` (visit each node once) |
| Space Complexity  | `O(h)` recursion stack; worst `O(n)`, average `O(log n)` |

---
