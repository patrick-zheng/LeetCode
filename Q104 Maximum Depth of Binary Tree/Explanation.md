# 🧩 LeetCode Problem: Maximum Depth of Binary Tree

- **Problem Link**: [104. Maximum Depth of Binary Tree – LeetCode](https://leetcode.com/problems/maximum-depth-of-binary-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/maximum-depth-of-binary-tree/solutions/)

---

## 🧠 Algorithm Explanation

The depth of a `null` node is `0`. For any non-null node, the maximum depth is:
  
`1 + max(depth(left), depth(right))`.

This naturally suggests a **recursive DFS**. Alternatively, you can perform a **BFS (level-order traversal)** and count how many levels you process.

---

### 🪜 Steps

1. **Base Case**: If `root` is `None`, return `0` (empty tree has depth 0).  
2. **Recurse**: Compute the maximum depth of the left and right subtrees.  
3. **Combine**: Return `1 + max(left_depth, right_depth)` to account for the current node’s level.

---

## ✅ Constraints / Notes

- Tree can be empty.
- Depth can be large for a skewed tree; recursion depth may hit Python’s recursion limit. Use BFS if concerned.
- Node values don’t affect depth (only structure matters).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | `O(n)`          |
| Space Complexity  | `O(h)` (DFS)    |
|                   | `O(w)` (BFS)    |

- `n`: number of nodes  
- `h`: height of the tree (recursion stack)  
- `w`: maximum width of the tree (queue size in BFS)

---
