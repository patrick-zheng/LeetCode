# 🧩 LeetCode Problem: Construct Binary Tree from Preorder and Inorder Traversal

- **Problem Link**: [Construct Binary Tree from Preorder and Inorder Traversal – LeetCode](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/solutions/)

---

## 🧠 Algorithm Explanation

Use `preorder` to pick each subtree’s root (order is `root → left → right`) and an index map over `inorder` (order is `left → root → right`) to split the current inorder range into left and right parts.  
Maintain a moving pointer `pre_idx` into `preorder`. For any inorder window `[lo..hi]`:

1. `root_val = preorder[pre_idx]` (this is the subtree root), then advance `pre_idx`.
2. Find `mid = in_pos[root_val]` in O(1) via a hashmap.
3. Recurse on `[lo..mid-1]` to build the left subtree, then `[mid+1..hi]` for the right.

This avoids slicing (which is O(n)) and repeated linear searches, yielding O(n) time overall.

---

### 🪜 Steps

1. Build a dictionary `in_pos[val] = index` for all values in `inorder`.
2. Initialize `pre_idx = 0`.
3. Define `build(lo, hi)`:
   - If `lo > hi`, return `None`.
   - Take `root_val = preorder[pre_idx]`; increment `pre_idx`.
   - `mid = in_pos[root_val]`.
   - Create `root` node.
   - `root.left  = build(lo, mid - 1)`
   - `root.right = build(mid + 1, hi)`
   - Return `root`.
4. Call `build(0, len(inorder) - 1)`.

---

## ✅ Constraints

- The tree contains **unique values** (required to split by `inorder`).
- `preorder` and `inorder` are valid traversals of the **same** binary tree.
- Typical limits: `1 ≤ n ≤ 3000` (you don’t need these exact numbers to implement the solution).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | O(n)              |
| Space Complexity  | O(n) (map + stack)|

---
