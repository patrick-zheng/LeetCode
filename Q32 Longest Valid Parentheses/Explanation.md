# 🧩 LeetCode Problem: Longest Valid Parentheses

- **Problem Link**: [Longest Valid Parentheses – LeetCode](https://leetcode.com/problems/longest-valid-parentheses/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-valid-parentheses/solutions/)

---

## 🧠 Algorithm Explanation

We use a **stack-based approach** to solve this problem efficiently in a single pass.

- The idea is to use a stack to keep track of indices of unmatched `'('`.
- By storing indices instead of characters, we can compute the length of valid substrings directly.
- We initialize the stack with `-1` to serve as a base index for valid lengths.

This approach is chosen because:

- It handles nested and adjacent valid substrings effectively.
- It avoids unnecessary backtracking or nested loops.

---

### 🪜 Steps

1. **Initialize a stack with `-1`**:
   - Acts as the base index for the first potential valid substring.

2. **Iterate over the string**:
   - If the character is `'('`, push its index onto the stack.
   - If the character is `')'`:
     - Pop the top of the stack.
     - If the stack is empty after popping, push the current index as a new base.
     - Otherwise, calculate the length of the current valid substring using `i - stack[-1]` and update `max_len`.

3. **Return `max_len`**:
   - This will be the length of the longest valid parentheses substring.

---

## ✅ Constraints

- `0 <= s.length <= 3 * 10⁴`
- `s[i]` is either `'('` or `')'`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

- `n` is the length of the input string.
- The stack holds at most `n` indices in the worst case (e.g., all `'('`).

---
