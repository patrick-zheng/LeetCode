# 🧩 LeetCode Problem: Pow(x, n)

- **Problem Link**: [Pow(x, n) – LeetCode](https://leetcode.com/problems/powx-n/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/powx-n/solutions/)

---

## 🧠 Algorithm Explanation

This solution directly uses Python's built-in exponentiation operator `**` to calculate \( x^n \).  
Python's floating-point arithmetic handles both positive and negative exponents, as well as zero exponents.  
Although this is the simplest possible implementation, it's not the most optimal for very large `n` compared to methods like **binary exponentiation**.

---

### 🪜 Steps

1. **Step 1**: Take input `x` (a float) and `n` (an integer).
2. **Step 2**: Use Python's `**` operator to compute \( x^n \).
3. **Step 3**: Return the computed result.

---

## ✅ Constraints

- \(-100.0 < x < 100.0\)  
- \( -2^{31} \le n \le 2^{31} - 1 \)  
- \(-10^4 \le x^n \le 10^4\)  
- `n` is an integer.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(1)       |
| Space Complexity  | O(1)       |

---
