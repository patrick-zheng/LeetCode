# 🧩 LeetCode Problem: Climbing Stairs

- **Problem Link**: [Climbing Stairs – LeetCode](https://leetcode.com/problems/climbing-stairs/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/climbing-stairs/solutions/)

---

## 🧠 Algorithm Explanation

Let `f(n)` be the number of distinct ways to reach step `n`.  
From step `n`, the last move must have come from:

- step `n-1` with a 1-step jump, or
- step `n-2` with a 2-step jump.

Therefore,
\[
f(n)=f(n-1)+f(n-2),
\]
with base cases `f(1)=1`, `f(2)=2`.  
This is the Fibonacci recurrence. We compute iteratively while keeping only the last two values for **O(1)** space.

---

### 🪜 Steps

1. **Base cases**:  
   If `n <= 2`, return `n`.
2. **Iterate** from `3` to `n`, updating two rolling variables:  
   `prev2 = f(i-2)`, `prev1 = f(i-1)`, then `curr = prev1 + prev2`.
3. **Return** `curr` (or the last stored value after the loop).

---

## ✅ Constraints

- `1 ≤ n ≤ 45`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
