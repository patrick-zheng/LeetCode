# 🧩 LeetCode: Recover Binary Search Tree

- **Problem**: Two nodes of a BST were swapped by mistake. Recover the tree **without** changing its structure.

---

## 🧠 Key Idea

In a valid BST, an **in-order traversal** yields a **sorted** sequence. If two nodes are swapped:

- There will be either **one** inversion (adjacent swap) or **two** inversions (non-adjacent swap) in the in-order sequence.
- Track the first node of the **first** inversion (`first`) and the second node of the **last** inversion (`second`), then **swap their values**.

You can do this with:

- **Recursion / Stack (O(h) space)**, or
- **Morris Traversal (O(1) space)** by temporarily threading the tree.

---

### 🪜 Steps

1. Traverse the tree **in-order**.
2. Maintain a pointer `prev` to the previously visited node.
3. Whenever `prev.val > curr.val`:
   - If `first` is not yet set → set `first = prev`.
   - Always set/update `second = curr`.
4. After traversal, **swap** `first.val` and `second.val`.

---

## ✅ Constraints

- The tree contains **at least two nodes**.
- Exactly **two nodes’ values** are swapped.
- Tree structure must remain unchanged.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time              | O(n)              |
| Space (recursive) | O(h) (call stack) |
| Space (Morris)    | O(1)              |

---
