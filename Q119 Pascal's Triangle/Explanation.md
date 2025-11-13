# 🧩 LeetCode Problem: Pascal's Triangle II

- **Problem Link**: [Pascal's Triangle II – LeetCode](https://leetcode.com/problems/pascals-triangle-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/pascals-triangle-ii/solutions/)

---

## 🧠 Algorithm Explanation

We want the `rowIndex`-th (0-based) row of Pascal's Triangle without generating the whole triangle.  
The key property of Pascal's Triangle is:

Each element (except the edges) is the sum of the two elements directly above it:
`row[j] = row[j] + row[j - 1]` (when updating in-place from right to left).

We use a single list `row` of length `rowIndex + 1`, initialized with all `1`s. Then, for each intermediate row `i`, we update `row` **from right to left** so that we don't overwrite values we still need. After finishing, `row` directly contains the desired row.

---

### 🪜 Steps

1. **Step 1**:  
   Initialize a list `row` of length `rowIndex + 1` with all values set to `1`. This already satisfies the first and last elements being `1` for any row.

2. **Step 2**:  
   Loop `i` from `1` to `rowIndex - 1`. For each `i`, treat the current content of `row` as the `i`-th row of Pascal's Triangle and prepare to update it into the `(i + 1)`-th row.

3. **SStep 3**:  
   For each `i`, loop `j` from `i` down to `1` (right to left), and update:
   `row[j] = row[j] + row[j - 1]`.  
   This applies Pascal's rule using the previous row's values (which are still intact because we go right to left).  
   After all iterations, return `row` as the final answer.

---

## ✅ Constraints

- `0 <= rowIndex <= 33`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity        |
|-------------------|-------------------|
| Time Complexity   | O(rowIndex²)      |
| Space Complexity  | O(rowIndex)       |

---
