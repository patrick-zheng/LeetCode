# 🧩 LeetCode Problem: Symmetric Tree (Iterative)

- **Problem Link**: [Symmetric Tree – LeetCode](https://leetcode.com/problems/symmetric-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/symmetric-tree/solutions/)

---

## 🧠 Algorithm Explanation

Use a **queue** to compare nodes in mirrored pairs. Start with `(root.left, root.right)`.  
While the queue isn’t empty, pop a pair `(a, b)`:

- If both are `null`, continue.
- If exactly one is `null` or `a.val != b.val`, the tree isn’t symmetric.
- Otherwise, push children in **mirror order**:
  - `(a.left,  b.right)`
  - `(a.right, b.left)`

If the loop completes without mismatch, the tree is symmetric.

---

### 🪜 Steps

1. **Initialize**  
   - If `root` is `null`, return `true`.  
   - Create a queue and push `(root.left, root.right)`.

2. **Process pairs**  
   - While queue not empty:
     - Pop `(a, b)`.
     - If `a == null && b == null`, `continue`.
     - If `a == null || b == null || a.val != b.val`, return `false`.

3. **Enqueue mirror children**  
   - Push `(a.left,  b.right)` and `(a.right, b.left)`.

4. **Return**  
   - If no mismatches were found, return `true`.

---

## ✅ Constraints

- Binary tree nodes with `left` and `right` pointers.
- Values compared by equality only.
- Handles large/unbalanced trees without recursion limits.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
