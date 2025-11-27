# 🧩 LeetCode Problem: Sum Root to Leaf Numbers

- **Problem Link**: [Sum Root to Leaf Numbers – LeetCode](https://leetcode.com/problems/sum-root-to-leaf-numbers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/)

---

## 🧠 Algorithm Explanation

Each **root-to-leaf path** represents a number formed by concatenating the digits along the path.

Example:  
Path `1 -> 2 -> 3` represents the number `123`.

We can use **DFS (Depth-First Search)** to traverse from the root to all leaves, and carry the number formed so far:

- At each node, given a partial number `current`, the updated value is:
  \[
  \text{current} = current \times 10 + node.val
  \]
- When we hit a **leaf** (no left or right child), that `current` is one complete root-to-leaf number and contributes directly to the final sum.
- For internal nodes, we recursively compute the sum of the left and right subtrees.

This approach is natural for trees and visits each node exactly once.

---

### 🪜 Steps

1. **Define a DFS helper** `dfs(node, current)`:
   - If `node` is `null`, return `0` (no contribution).

2. **Update the path number**:
   - Compute `current = current * 10 + node.val`.

3. **Check if node is a leaf**:
   - If `node.left == null` and `node.right == null`, return `current` (a complete root-to-leaf number).

4. **Recurse on children**:
   - Return `dfs(node.left, current) + dfs(node.right, current)`.

5. From the main function `sumNumbers(root)`, call `dfs(root, 0)` and return the result.

---

## ✅ Constraints

- Each node’s value is a digit: `0 <= Node.val <= 9`.
- The tree has a finite number of nodes (e.g. up to around `10^4` in typical constraints).
- The final sum fits in a **32-bit signed integer**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|-----------|
| Time Complexity   | `O(N)`    |
| Space Complexity  | `O(H)`    |

- `N` = number of nodes in the tree (each node visited once).
- `H` = height of the tree (recursion depth):
  - Worst case (skewed tree): `O(N)`
  - Balanced tree: `O(log N)`

---
