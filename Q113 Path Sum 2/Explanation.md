# 🧩 LeetCode Problem: Path Sum II

- **Problem Link**: [Path Sum II – LeetCode](https://leetcode.com/problems/path-sum-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/path-sum-ii/solutions/)

---

## 🧠 Algorithm Explanation

Use **DFS with backtracking**. Traverse from root to leaves, maintaining:

- a running **path** (list of node values), and
- a **remaining sum** (`targetSum - sum(path)`).

When you reach a **leaf** and `remaining == leaf.val`, record a **copy** of the current path. Backtrack (pop the last node) after exploring each subtree so the path stays correct for sibling branches.

Why DFS + backtracking?

- DFS naturally explores root→leaf paths.
- Backtracking avoids rebuilding lists from scratch and keeps memory usage to O(height) besides the output.

---

### 🪜 Steps

1. **Handle empty tree**: if `root == null`, return `[]`.
2. **DFS(node, remaining, path)**:
   - Append `node.val` to `path`.
   - If `node` is a **leaf** and `remaining == node.val`, append a **copy** of `path` to `answers`.
   - Recurse on `left` and `right` with `remaining - node.val`.
   - **Backtrack** by popping the last element from `path`.
3. **Return results** collected in `answers`.

---

## ✅ Constraints

- Number of nodes: `0 … 5000`
- Node values may be negative: `-1000 … 1000`
- `targetSum` may be negative: `-1000 … 1000`
- Binary tree not necessarily balanced

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(N + K) — visit each node once (O(N)) plus copying K total path elements across all outputs (worst-case can look like O(N²) in a skewed tree). |
| Space Complexity  | O(H) auxiliary for recursion + path, where H is tree height, **plus** the space for the returned paths. |

---
