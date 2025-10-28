# 🧩 LeetCode Problem: Construct Binary Tree from Inorder and Postorder Traversal

- **Problem Link**: [106. Construct Binary Tree from Inorder and Postorder Traversal – LeetCode](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/)

---

## 🧠 Algorithm Explanation

We use the properties of traversals:

- **Postorder** = Left → Right → Root ⇒ the **last** element is the current **root**.
- **Inorder** = Left → Root → Right ⇒ elements **left** of root form the **left** subtree, elements **right** form the **right** subtree.

Algorithm:

1. Keep a hash map from value → index in `inorder` for O(1) splits.
2. Walk a global pointer `p` from the end of `postorder` (root of current subtree).
3. Recursively build **right** subtree first, then **left** (since we’re consuming from `postorder`’s end: Root’s right subtree root appears just before Root’s value).

This yields an O(n) reconstruction with correct structure.

---

### 🪜 Steps

1. **Preprocess**
   - Build `idx = { value: inorder_index }`.
   - Initialize `p = postorder.length - 1`.

2. **Recursive Build(lo, hi)**
   - If `lo > hi`, return `null` (empty segment).
   - Let `rootVal = postorder[p--]`, create node.
   - Find `mid = idx[rootVal]` in `inorder`.

3. **Recurse (Right then Left)**
   - `root.right = Build(mid + 1, hi)`
   - `root.left  = Build(lo, mid - 1)`
   - Return `root`.

---

## ✅ Constraints

- `1 <= n <= 3000` (typical; exact limits vary by platform)
- All node values are **unique** (required to split `inorder` unambiguously).
- `inorder` and `postorder` contain the **same multiset** of values and have equal length.
- If duplicates could appear, extra disambiguation data would be needed (not covered by standard problem).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |
