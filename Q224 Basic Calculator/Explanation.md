# LeetCode Problem: Basic Calculator

- **Problem Link**: [Basic Calculator – LeetCode](https://leetcode.com/problems/basic-calculator/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/basic-calculator/solutions/)

---

## Algorithm

The expression has **non-negative integers**, **`+` / `-`**, **parentheses**, and
spaces. There is no `*` or `/`, so evaluation is **left-to-right** with the usual
grouping from parentheses.

### One pass with a stack (**O(n)** time)

Keep running parts:

- **`num`**: the integer currently being read digit by digit.
- **`sign`**: the unary effect of the last explicit operator (**`+1`** or
  **`-1`**) that applies to the **next** number or parenthesized subexpression.
- **`res`**: the sum of everything **to the left** inside the current
  parenthesis level, already folded with correct signs.

**Operators `+` / `-`:** flush the previous number into `res` with
`res += sign * num`, reset `num = 0`, and set `sign` to **`+1`** or **`-1`**.

**`(`:** the outer level is “paused”. Push **`res`** then **`sign`** (order
matches how we pop later), then start a fresh inner scope with **`res = 0`**,
**`sign = 1`**.

**`)`:** finish the inner number: `res += sign * num`, reset `num = 0`. Pop
**`prev_sign`** then **`prev_res`** and combine:

\[
\texttt{res} \leftarrow \texttt{prev\_res} + \texttt{prev\_sign} \cdot \texttt{res}
\]

which is exactly “outer partial sum ± (value of this parenthesis)”.

**Spaces:** ignore.

After the scan, add the last literal (if any): **`res + sign * num`**.

---

## Constraints

- `1 <= s.length <= 3 * 10^5`
- `s` consists of digits, `'+'`, `'-'`, `'('`, `')'`, and `' '`
- `s` represents a **valid** expression with **non-negative** integers in
  \([0, 2^{31} - 1]\)

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** — one left-to-right pass; each char does amortized **O(1)** work       |
| Space Complexity   | **O(n)** — stack holds suspended totals; depth is at most the opens in input    |

---
