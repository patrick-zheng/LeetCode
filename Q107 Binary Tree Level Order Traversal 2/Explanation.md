# 🧩 LeetCode Problem: Binary Tree Level Order Traversal II

- **Problem Link**: [Binary Tree Level Order Traversal II – LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-level-order-traversal-ii/solutions/)

---

## 🧠 Algorithm Explanation

We need a **bottom-up** level order of node values (from leaves up to the root).  
The most natural approach is **BFS (breadth-first search)** using a queue:

- Traverse the tree level by level from top to bottom (root to leaves).
- Collect values for each level.
- Finally **reverse** the list of levels to obtain bottom-up order (or prepend each level as you go).

Why BFS? It inherently groups nodes by level with a single pass and simple bookkeeping.

(Alternative: **DFS** with `depth` tracking—append values to `levels[depth]`, then reverse at the end.)

---

### 🪜 Steps

1. **Handle empty tree**: if `root` is `null`, return `[]`.
2. **Initialize queue** with `root`; prepare `levels = []`.
3. **Process level**:
   - Let `size = queue.length` (number of nodes in current level).
   - Pop `size` nodes, push their values into a `row`.
   - Enqueue each popped node’s non-null `left` and `right` children.
4. **Store level**: `levels.append(row)`.
5. **Repeat** until the queue is empty.
6. **Bottom-up**: return `reverse(levels)` (or build by inserting at the front each iteration).

---

## ✅ Constraints

- Tree node count `n` fits in typical memory limits.
- Node values can be any integers (no ordering assumptions).
- Works for skewed trees (acts like a list of single-element levels).
- Time/space proportional to the number of nodes and the widest level.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |
