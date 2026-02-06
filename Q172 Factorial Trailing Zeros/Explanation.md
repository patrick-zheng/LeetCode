# 🧩 LeetCode Problem: Factorial Trailing Zeroes

- **Problem Link**: [Factorial Trailing Zeroes – LeetCode](https://leetcode.com/problems/factorial-trailing-zeroes/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/factorial-trailing-zeroes/solutions/)

---

## 🧠 Algorithm Explanation

Trailing zeroes in a factorial come from factors of **10**, and  
\[
10 = 2 \times 5
\]

In `n!`, there are **far more factors of 2 than factors of 5**, so the number of trailing zeroes is determined entirely by how many times **5** appears as a factor.

Instead of computing the factorial (which is infeasible for large `n`), we **count how many multiples of 5, 25, 125, ...** are less than or equal to `n`.  
Each contributes one or more factors of 5.

---

### 🪜 Steps

1. **Step 1**: Initialize a counter to store the number of trailing zeroes.

2. **Step 2**: Repeatedly divide `n` by 5 and add the quotient to the counter.
   - `n // 5` counts numbers with at least one factor of 5
   - `n // 25` counts numbers with an extra factor of 5
   - `n // 125`, and so on

3. **Step 3**: Stop when `n` becomes 0 and return the counter.

---

## ✅ Constraints

- `0 ≤ n ≤ 10⁴`
- `n` is an integer
- Must not compute the factorial directly

---

## ⏱ Time and Space Complexity

| Metric            | Complexity    |
|-------------------|------------   |
| Time Complexity   | **O(log₅ n)** |
| Space Complexity  | **O(1)**      |

---

## 🧪 Example

For `n = 100`:
