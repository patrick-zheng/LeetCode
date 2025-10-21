# 🧩 LeetCode Problem: Binary Tree Level Order Traversal

- **Problem Link**: [Binary Tree Level Order Traversal – LeetCode](https://leetcode.com/problems/binary-tree-level-order-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/)

---

## 🧠 Algorithm Explanation

Use **Breadth-First Search (BFS)**. Maintain a queue to process nodes level by level. For each level, read the current queue size, pop exactly that many nodes, record their values, and push their non-null children. Append the collected values for each level to the answer.

---

### 🪜 Steps

1. **Initialize**: If `root` is `null`, return `[]`. Otherwise, push `root` into a queue and create an empty `result` list.
2. **Process a level**: Let `k = queue.length`. Create an empty `level` list. Repeat `k` times: pop a node, append its value to `level`, and enqueue its left/right children if present.
3. **Store & repeat**: Append `level` to `result`. Continue while the queue is not empty.

---

## ✅ Constraints

- Tree can be empty.
- Node values can be any integers.
- Let `n` be the number of nodes; all nodes are visited at most once.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
