# 🧩 LeetCode Problem: Unique Binary Search Trees

- **Problem Link**: [Unique Binary Search Trees – LeetCode](https://leetcode.com/problems/unique-binary-search-trees/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/unique-binary-search-trees/solutions/)

---

## 🧠 Algorithm Explanation

The problem asks for the number of structurally unique Binary Search Trees (BSTs) that can be formed using values `1...n`.

This is a **Dynamic Programming** problem based on the **Catalan Number** formula.

For each number `i` as the root:

- The left subtree contains `i - 1` nodes.
- The right subtree contains `n - i` nodes.
  
The total number of unique BSTs for `n` nodes is the sum over all possible roots:
\[
G(n) = \sum_{i=1}^{n} G(i-1) \times G(n-i)
\]

Where:

- `G(0) = 1` (empty tree)
- `G(1) = 1` (single node tree)

---

### 🪜 Steps

1. **Initialize Base Cases**  
   Set `G[0] = 1` and `G[1] = 1` since there’s exactly one unique BST for 0 or 1 node.

2. **Iterate Through Node Counts**  
   For each number of nodes `n = 2...N`, calculate `G[n]` using the formula above.

3. **Iterate Through Possible Roots**  
   For each possible root, multiply the number of unique left and right subtrees and sum them.

4. **Return the Result**  
   The final result `G[n]` gives the number of unique BSTs that can be built with `n` distinct nodes.

---

## ✅ Constraints

- \( 1 \leq n \leq 19 \)
- Result fits in a 32-bit integer.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| **Time Complexity**   | \( O(n^2) \) |
| **Space Complexity**  | \( O(n) \)   |

---
