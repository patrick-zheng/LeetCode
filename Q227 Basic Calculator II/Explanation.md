# LeetCode Problem: Basic Calculator II

- **Problem Link**: [Basic Calculator II – LeetCode](https://leetcode.com/problems/basic-calculator-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/basic-calculator-ii/solutions/)

---

## Algorithm

Evaluate a string with **non-negative integers**, **`+` / `-` / `*` / `/`**, and
spaces. There are **no parentheses**.

### One pass, **O(1)** extra space

Keep:

- **`result`**: everything strictly to the left of the current term, already
  folded by earlier **`+`** / **`-`** decisions.
- **`last_number`**: the current term after applying any pending **`*`** /
  **`/`** to the last operand in that term.

Scan left to right and parse the current literal into **`curr`**. Track
**`op`**, the operator **before** **`curr`**; start with **`'+'`** so the first
number is treated as **`+curr`**.

Whenever you hit an operator or the **last index** (to flush the final number):

1. **`+`**: **`result += last_number`**, then **`last_number = curr`**.
2. **`-`**: same, but **`last_number = -curr`**.
3. **`*`**: **`last_number *= curr`**.
4. **`/`**: divide with **truncation toward zero** (Python **`int`** of true
   division; C++ and Rust **integer `/`** truncate toward zero).

If the current character is an operator, set **`op`** to it; always reset
**`curr = 0`**.

Return **`result + last_number`**.

Same logic as a **stack** of terms: **`+`** / **`-`** push a finished term into
the accumulated sum; **`*`** / **`/`** only combine with the top term.

---

## Constraints

- `1 <= s.length <= 3 * 10^5`
- `s` consists of digits, `'+'`, `'-'`, `'*'`, `'/'`, and spaces
- All integers in the expression are non-negative and in \([0, 2^{31} - 1]\)
- The expression is valid and the result fits in a 32-bit signed integer

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — single scan; O(1) work per character                                 |
| Space Complexity   | **O(1)** — only a few integer variables (no auxiliary stack)                    |

---
