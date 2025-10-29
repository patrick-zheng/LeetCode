# 🧩 LeetCode Problem: Convert Sorted Array to Binary Search Tree

- **Problem Link**: [Convert Sorted Array to Binary Search Tree – LeetCode](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/)

---

## 🧠 Algorithm Explanation

Pick the **middle element** of the (sorted) array as the root. Recursively build the left subtree from the left half and the right subtree from the right half. This keeps subtree sizes within one of each other, ensuring a **height-balanced** BST, and preserves the in-order (sorted) property.

---

### 🪜 Steps

1. **Base Case**: If the current subarray is empty, return `null`.
2. **Choose Middle**: `mid = (lo + hi) // 2` and create a node with `nums[mid]`.
3. **Recurse**:
   - Left subtree from `[lo, mid - 1]`
   - Right subtree from `[mid + 1, hi]`
4. **Return** the node.

---

## ✅ Constraints

- `nums` is sorted in **ascending** order.
- `0 ≤ n = len(nums)`
- Output must be a **height-balanced** BST (left/right subtree heights differ by ≤ 1).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | `O(n)` (each element becomes a node once) |
| Space Complexity  | `O(log n)` average recursion depth (worst-case call stack `O(n)`) |

---
