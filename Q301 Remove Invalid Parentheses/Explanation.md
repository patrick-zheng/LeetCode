# LeetCode Problem: Remove Invalid Parentheses

- **Problem Link**: [Remove Invalid Parentheses – LeetCode](https://leetcode.com/problems/remove-invalid-parentheses/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/remove-invalid-parentheses/solutions/)

---

## Algorithm

Given `s` with `'('`, `')'`, and lowercase letters, remove the **minimum**
number of parentheses so every remaining string is **valid**. Return all
distinct valid strings (any order).

### Step 1 — count mandatory removals

Scan left to right:

- `'('` increases a running open count.
- `')'` either cancels one open paren or counts as one extra `')'` to remove.

After the scan, `left_remove` is unmatched `'('` and `right_remove` is
unmatched `')'`. Any optimal answer removes exactly those counts (no more, no
less).

### Step 2 — backtracking

DFS index by index with state `(open_count, close_count, left_rem, right_rem)`:

1. **Letters** — always keep.
2. **Remove** — skip `'('` if `left_rem > 0`, or `')'` if `right_rem > 0`.
3. **Keep** — append `'('`, or append `')'` only when `close_count < open_count`
   (stays valid while building).

At the end, record the path when both removal budgets are zero. A `set` (or
`HashSet`) deduplicates identical strings produced by different delete orders.

Example: `s = "()())"` → `["()()", "(())"]`.

---

## Constraints

- `1 <= s.length <= 25`
- `s[i]` is `'('`, `')'`, or a lowercase English letter

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(2^n)** worst case — branch per paren; `n <= 25` keeps search bounded        |
| Space Complexity   | **O(n)** — recursion depth and path buffer; result set may grow large           |

---
<!-- markdownlint-enable MD013 -->
