# 🧩 LeetCode Problem: Binary Tree Maximum Path Sum

- **Problem Link**: [Binary Tree Maximum Path Sum – LeetCode](https://leetcode.com/problems/binary-tree-maximum-path-sum/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-maximum-path-sum/solutions/)

---

## 🧠 Algorithm Explanation

We need the **maximum sum over all simple paths** in a binary tree. A path:

- Can start and end at **any node** (not necessarily the root),
- Follows parent–child edges,
- Cannot visit a node more than once.

For each node, we consider:

1. **Best “one-side” path that can extend upward to its parent**  
   From a parent’s point of view, it can only continue through **one child** (left or right).  
   So we compute, for each node:
   $$
   \text{one\_side\_gain} = \text{node.val} + \max(0,\ \text{left\_gain},\ \text{right\_gain})
   $$
   We use `max(0, ...)` because negative paths only make things worse, so we drop them.

2. **Best complete path that uses this node as the “peak”**  
   This can include both children:
   $$
   \text{path\_through} = \text{node.val} + \max(0,\text{left\_gain}) + \max(0,\text{right\_gain})
   $$
   This is a candidate for the global maximum.

We perform a DFS:

- At each node, recursively compute the best downward gain from left and right.
- Update a **global maximum** with the best `path_through` for this node.
- Return to the parent the best **one-side** gain (`node.val + max(left_gain, right_gain)`).

---

### 🪜 Steps

1. **Initialize global maximum**  
   - Set `max_sum = -∞` (e.g. `float("-inf")`) to track the best path sum found so far.

2. **Depth-first search (DFS)**  
   - Define `dfs(node)`:
     - If `node` is `None`, return `0`.
     - Recursively compute:
       - `left_gain = max(0, dfs(node.left))`
       - `right_gain = max(0, dfs(node.right))`
     - Compute the best path that uses `node` as the highest point:  
       `path_through = node.val + left_gain + right_gain`
     - Update global maximum:  
       `max_sum = max(max_sum, path_through)`

3. **Return one-side gain to parent**  
   - From `dfs(node)`, return the best gain that can continue upward:  
     `return node.val + max(left_gain, right_gain)`.
   - After calling `dfs(root)`, `max_sum` holds the answer. Return `max_sum`.

---

## ✅ Constraints

- `1 <= number of nodes <= 3 * 10^4`
- `-1000 <= Node.val <= 1000`
- Tree is arbitrary (not necessarily balanced or a BST).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | O(n)            |
| Space Complexity  | O(h) recursion, where h is tree height (O(n) worst case) |

---
