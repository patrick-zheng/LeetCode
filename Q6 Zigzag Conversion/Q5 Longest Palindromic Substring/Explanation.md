# 🧩 LeetCode Problem: Zigzag Conversion

- **Problem Link**: [Zigzag Conversion – LeetCode](https://leetcode.com/problems/zigzag-conversion/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/zigzag-conversion/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using a **simulation-based traversal** that builds each row of the zigzag pattern line by line.

### Why This Approach?

- Instead of simulating the full 2D zigzag pattern, we can directly append characters to the corresponding row strings.
- We track the **current row** and **direction of movement** to assign each character appropriately.
- This yields an **O(n)** solution in both time and space.

---

### 🪜 Steps

1. **Handle Edge Case**:
   If `numRows` is 1 or the input string is shorter than `numRows`, return the original string (no zigzag needed).

2. **Initialize Rows**:
   Create a list of strings (`rows`) where each string will collect characters for a specific row.

3. **Simulate the Zigzag Pattern**:
   - Use a pointer `curr_row` to track the current row index.
   - Use a flag `going_down` to change direction when the top or bottom is reached.
   - Iterate over each character in the input string:
     - Append the character to the current row.
     - Change direction if the current row is the first or last row.
     - Move `curr_row` up or down accordingly.

4. **Concatenate Rows**:
   Combine all row strings into a single result string by joining them.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

- `n = len(s)`
- Each character is visited once and stored once.

---
