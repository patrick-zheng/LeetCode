# 🧩 LeetCode Problem: Gray Code

- **Problem Link**: [Gray Code – LeetCode](https://leetcode.com/problems/gray-code/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/gray-code/solutions/)

---

## 🧠 Algorithm Explanation

We want to generate a valid n-bit Gray code sequence of length \(2^n\). A Gray code is an ordering of all \(n\)-bit numbers such that consecutive numbers differ in exactly one bit, and the first and last numbers also differ in one bit.

The optimal way to generate Gray codes uses the formula:

\[
g(i) = i \oplus (i \gg 1)
\]

where:

- \(i\) ranges from \(0\) to \(2^n - 1\),
- \(\oplus\) is the bitwise XOR operator,
- \(\gg\) is the right-shift operator.

This guarantees that each successive code differs by exactly one bit and covers all required numbers.

---

### 🪜 Steps

1. **Iterate** from \(i = 0\) to \(2^n - 1\).
2. **Transform** each integer using the Gray code formula: \(i ^ (i >> 1)\).
3. **Collect** the transformed values in a list to form the sequence.

---

## ✅ Constraints

- \(1 \leq n \leq 16\)
- Sequence length = \(2^n\), which is feasible since maximum length is \(65{,}536\).

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | \(O(2^n)\) |
| Space Complexity  | \(O(2^n)\) (for storing the result) |

---
