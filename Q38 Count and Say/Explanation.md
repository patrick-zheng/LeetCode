# 🧩 LeetCode Problem: Count and Say

- **Problem Link**: [Count and Say – LeetCode](https://leetcode.com/problems/count-and-say/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/count-and-say/solutions/)

---

## 🧠 Algorithm Explanation

This problem involves generating the `n`-th term of the "Count and Say" sequence, where each term is derived by describing the previous term.

The idea is to "say" the digits of the previous term out loud: count how many times a digit appears consecutively, then say the digit. For example:

- Term 1: `"1"`
- Term 2: "one 1" → `"11"`
- Term 3: "two 1s" → `"21"`
- Term 4: "one 2, one 1" → `"1211"`
- and so on...

We use recursion to compute the previous term and process it by grouping consecutive digits.

---

### 🪜 Steps

1. **Step 1**: If `n == 1`, return `"1"` as the base case.
2. **Step 2**: Recursively get the `(n - 1)`-th term.
3. **Step 3**: Loop through the previous term and:
   - Count consecutive identical digits.
   - Append the count and the digit to the result when the sequence breaks.
4. **Step 4**: Join the result list into a string and return it.

---

## ✅ Constraints

- `1 <= n <= 30`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity    |
|-------------------|---------------|
| Time Complexity   | O(2ⁿ) approx. |
| Space Complexity  | O(2ⁿ) approx. |

> The length of each term increases roughly exponentially, and recursive calls add stack space.

---
