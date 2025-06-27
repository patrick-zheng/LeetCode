# 🧩 LeetCode Problem: Valid Parentheses

- **Problem Link**: [Valid Parentheses – LeetCode](https://leetcode.com/problems/valid-parentheses/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-parentheses/solutions/)

---

## 🧠 Algorithm Explanation

We use a **stack** to keep track of opening brackets. Each time we see an opening bracket, we push it onto the stack. When we see a closing bracket, we check if it matches the top of the stack. If it doesn't, the string is invalid.

The stack ensures that brackets are closed in the correct order. If the stack is empty at the end, it means all brackets matched correctly.

---

### 🪜 Steps

1. **Initialize an empty stack**.
2. **Iterate** over each character in the string:
   - If it is an **opening bracket** (`(`, `{`, `[`), **push** it onto the stack.
   - If it is a **closing bracket** (`)`, `}`, `]`):
     - Check if the stack is empty → return `False`.
     - Pop from the stack and check if it **matches** the correct opening bracket.
     - If not → return `False`.
3. **After the loop**, check if the stack is empty:
   - If it is → return `True` (valid).
   - Otherwise → return `False` (some unmatched opening brackets).

---

## ✅ Constraints

- `1 <= s.length <= 10⁴`
- `s` consists only of characters `'(', ')', '{', '}', '[' and ']'`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
