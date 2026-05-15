# LeetCode Problem: Different Ways to Add Parentheses

- **Problem Link**: [Different Ways to Add Parentheses – LeetCode](https://leetcode.com/problems/different-ways-to-add-parentheses/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/different-ways-to-add-parentheses/solutions/)

---

## Algorithm

Every fully parenthesized expression evaluates its **outermost** operator last.
That operator splits the string into a left subexpression and a right
subexpression; recurse on both sides and combine every left value with every
right value using that operator.

### Divide and conquer + memoization

1. If the substring has **no** `+`, `-`, or `*`, parse it as one integer and
   return a singleton list.
2. Else, try each operator position as the outermost split:
   - Collect all values from the left substring and all values from the right
     substring.
   - For each pair `(a, b)`, append `a op b`.
3. Memoize answers keyed by **substring text** so repeated subexpressions are
   computed once.

Induction on substring length proves completeness: any parenthesization picks
exactly one outer operator matching one split above.

---

## Constraints

- `1 <= expression.length <= 20`
- `expression` contains digits and `'+'`, `'-'`, `'*'`.
- Literal integers lie in `[0, 999]`.

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **Exponential** in the worst case; memoization caches each substring once.      |
| Space Complexity   | **O(n^2)** memo entries for distinct substrings, plus recursion depth **O(n)**. |

---
