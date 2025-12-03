# 🧩 LeetCode Problem: Clone Graph

- **Problem Link**: [Clone Graph – LeetCode](https://leetcode.com/problems/clone-graph/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/clone-graph/solutions/)

---

## 🧠 Algorithm Explanation

We must return a **deep copy** of a connected, undirected graph starting from a given node.  
The tricky part is that the graph can contain **cycles** and shared neighbors, so a naive recursive copy could either:

- Loop forever, or  
- Create multiple copies of the same node.

To avoid this, we keep a **hash map** from each original node to its cloned node:

> `original_node -> cloned_node`

We then traverse the graph (DFS or BFS).  
For each node:

1. If it has already been cloned (exists in the map), we reuse that clone.
2. Otherwise, we:
   - Create a new node with the same value.
   - Store it in the map.
   - Recursively/iteratively clone all its neighbors and append their clones to this new node’s neighbor list.

This ensures each original node is cloned exactly once and all neighbor relations (edges) are preserved.

---

### 🪜 Steps

1. **Check for empty graph**  
   If the input `node` is `null`, return `null` immediately (nothing to clone).

2. **Create a mapping structure**  
   Use a hash map / dictionary to store:
   `mp[original] = clone`.

3. **DFS cloning procedure**  
   - Define a function `clone(curr)`:
     1. If `curr` is in `mp`, return `mp[curr]` (already cloned).
     2. Otherwise:
        - Create `copy` with value `curr.val`.
        - Save `mp[curr] = copy`.
        - For each neighbor `nei` in `curr.neighbors`, call `clone(nei)` and append the result to `copy.neighbors`.
        - Return `copy`.

---

## ✅ Constraints

- Number of nodes `n`: `1 ≤ n ≤ 100`
- Node values: `1 ≤ val ≤ 100`, all values are unique
- The graph is **connected** (every node is reachable from the given node)
- The graph is **undirected** and may contain **cycles**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity  |
|-------------------|------------|
| Time Complexity   | `O(V + E)` |
| Space Complexity  | `O(V)`     |

- `V` = number of nodes, `E` = number of edges.  
- Each node and edge is processed at most once.  
- We store at most one clone per node in the map and recursion/queue stack is also `O(V)`.

---
