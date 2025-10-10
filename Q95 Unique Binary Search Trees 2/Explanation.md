# 🧩 LeetCode Problem: Unique Binary Search Trees II

- **Problem Link**: [Unique Binary Search Trees II – LeetCode](https://leetcode.com/problems/unique-binary-search-trees-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/unique-binary-search-trees-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need to generate all structurally unique binary search trees (BSTs) that store values `1 … n`.  
The key insight is that for each number `i` in `[1, n]`, we can make it the root, then:

- All values `< i` form the **left subtree**.
- All values `> i` form the **right subtree**.

By recursively constructing all possible left and right subtrees, and combining them, we obtain every possible BST structure.  
To avoid recomputation of overlapping subproblems, we use **memoization**.

This approach leverages **divide and conquer** with caching.

---

### 🪜 Steps

1. **Base case**:  
   If the interval is empty (`lo > hi`), return `[None]` representing an empty tree.

2. **Choose root**:  
   For each `root_val` in `[lo, hi]`, treat it as the root.

3. **Build subtrees**:  
   - Recursively generate all valid left subtrees from `[lo, root_val - 1]`.  
   - Recursively generate all valid right subtrees from `[root_val + 1, hi]`.

4. **Combine**:  
   For each pair `(L, R)` of left and right subtree, create a new root node with `root_val`.

5. **Cache results**:  
   Store results for each `(lo, hi)` interval using memoization so repeated calls are efficient.

---

## ✅ Constraints

- `1 <= n <= 8`  
- Output size grows as the **Catalan number** `C_n`, which increases rapidly.  

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(C_n * n)** — we generate Catalan number of trees and try `n` roots at each step |
| Space Complexity  | **O(C_n * n)** — to store all tree structures plus recursion stack `O(n)` |

---
