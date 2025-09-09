# 🧩 LeetCode: Simplify Path

- **Problem Link**: [Simplify Path – LeetCode](https://leetcode.com/problems/simplify-path/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/simplify-path/solutions/)

---

## 📌 Problem Summary

Given an **absolute** Unix-style path string (always starting with `/`), return its **simplified canonical path** following these rules:

- `.` → current directory (ignore it)
- `..` → parent directory (pop one level if possible)
- Multiple slashes collapse into one
- Any other sequence (e.g., `...`, `....`) is a **valid name**, not special
- The result:
  - starts with exactly one `/`
  - has single `/` between components
  - has no trailing `/` unless it is just `/`
  - contains no `.` or `..` components

---

## 🧠 Approach

Use a **stack** to process components:

1. Split the path by `'/'`.
2. Skip `""` (from `//`) and `"."`.
3. If the component is `".."` → pop from the stack if non-empty.
4. Otherwise, push the component (treat names like `"..."` as normal directories).
5. Join the stack with `'/'` and prefix with `/`. If empty, return `/`.

**Why this works:** Only `.` and `..` are special; splitting handles collapsing multiple slashes naturally.

---

## 🪜 Steps

1. **Split** the input string on `'/'` to get components.
2. **Iterate** over components:
   - If `""` or `"."` → **continue**.
   - If `".."` → **pop** from stack if possible.
   - Else → **push** component onto stack.
3. **Build** the canonical path as `"/" + "/".join(stack)"`; if the stack is empty, return `"/"`.

---

## ⏱ Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

`n` = length of the input path.

---
