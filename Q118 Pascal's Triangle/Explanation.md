# 🧩 LeetCode Problem: Pascal's Triangle

- **Problem Link**: [Pascal's Triangle – LeetCode](https://leetcode.com/problems/pascals-triangle/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/pascals-triangle/solutions/)

---

## 🧠 Algorithm Explanation

We build the triangle row by row:

- Row 1 is `[1]`.
- Each subsequent row starts and ends with `1`.
- Every interior value is the sum of the two numbers directly above it in the previous row:
  `row[i] = prev[i-1] + prev[i]` for `1 ≤ i < len(prev)`.

This uses only the immediately previous row to construct the next row.

---

### 🪜 Steps

1. **Initialize** an empty list `triangle`.
2. **Loop** from `r = 0` to `numRows - 1`:
   - If `r == 0`, append `[1]`.
   - Otherwise, let `prev = triangle[-1]`, form the middle values by summing adjacent pairs in `prev`, then push `[1] + mid + [1]`.
3. **Return** `triangle`.

---

## ✅ Constraints

- Typical LeetCode constraints: `1 ≤ numRows ≤ 30`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | O(numRows²)     |
| Space Complexity  | O(numRows²)     |

The total number of elements across the first `numRows` rows is ~ `numRows*(numRows+1)/2`.

---
