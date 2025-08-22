# 🧩 LeetCode Problem: Spiral Matrix II

- **Problem Link**: [Spiral Matrix II – LeetCode](https://leetcode.com/problems/spiral-matrix-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/spiral-matrix-ii/solutions/)

---

## 🧠 Algorithm Explanation

We build an `n × n` matrix and fill it in a clockwise spiral order using four moving boundaries: `top`, `bottom`, `left`, and `right`.  
At each iteration we:

1. Fill the top row left→right, then move `top` down.  
2. Fill the right column top→bottom, then move `right` left.  
3. If rows remain, fill the bottom row right→left, then move `bottom` up.  
4. If columns remain, fill the left column bottom→top, then move `left` right.

This boundary-shrinking approach is simple, avoids revisiting cells, and naturally places numbers `1..n²` in spiral order.

---

### 🪜 Steps

1. **Initialize**
   - Create an `n × n` matrix of zeros.
   - Set `left=0`, `right=n-1`, `top=0`, `bottom=n-1`, and `num=1`.

2. **Spiral Fill**
   - Fill the **top row** from `left` to `right`; increment `top`.
   - Fill the **right column** from `top` to `bottom`; decrement `right`.
   - If `top ≤ bottom`, fill the **bottom row** from `right` to `left`; decrement `bottom`.
   - If `left ≤ right`, fill the **left column** from `bottom` to `top`; increment `left`.

3. **Repeat**
   - Continue while `left ≤ right` **and** `top ≤ bottom` until all numbers up to `n²` are placed.

---

## ✅ Constraints

- `n` is a positive integer.
- Output matrix must contain integers `1` through `n²` exactly once in clockwise spiral order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n²)      |
| Space Complexity  | O(1) extra (excluding the output matrix) |

---
