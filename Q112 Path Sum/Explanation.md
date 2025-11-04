# 🧩 LeetCode Problem: Path Sum (Root-to-Leaf)

- **Problem Link**: [Path Sum – LeetCode](https://leetcode.com/problems/path-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/path-sum/solutions/)

---

## 🧠 Algorithm Explanation

We use **depth-first search (DFS)** from the root, carrying a running remainder of the sum we still need.  
At each node, subtract the node’s value from `targetSum`.  
If we reach a **leaf** (no children) and the remainder is **0**, a valid path exists → return `true`.  
Otherwise, continue DFS on the left and right children with the updated remainder.

Why DFS?  
It naturally explores root-to-leaf paths and stops early as soon as a valid path is found, giving linear time over nodes.

---

### 🪜 Steps

1. **Handle empty tree**: If `root` is `null`, return `false`.
2. **Update remainder**: `remain = targetSum - root.val`.
3. **Leaf check**: If `root.left` and `root.right` are both `null`, return `remain == 0`.
4. **Recurse**: Return `hasPathSum(root.left, remain) || hasPathSum(root.right, remain)`.

---

## ✅ Constraints

- Input is a **binary tree**; path must be **root-to-leaf** (not just any root-to-node).
- Node values and `targetSum` are integers (values can be negative/zero/positive).
- Tree size `N` can be large; solution must be **O(N)**.
- Recursion depth equals tree height `H`; consider stack limits for very skewed trees (iterative stack can be used if needed).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|----------------|
| Time Complexity   | O(N)           |
| Space Complexity  | O(H) recursion (worst O(N), best O(log N) for balanced trees) |

---
