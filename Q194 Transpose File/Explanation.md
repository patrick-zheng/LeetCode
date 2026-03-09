# 🧩 LeetCode Problem: Transpose File

- **Problem Link**: [Transpose File – LeetCode](https://leetcode.com/problems/transpose-file/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/transpose-file/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to transform the rows of the file into columns. Since each line contains the same number of space-separated fields, we can process the file row by row and build the transposed result column by column.

The optimal approach uses `awk`, which is well-suited for structured text processing. As `awk` reads each line, it splits it into fields automatically. We then append each field to its corresponding transposed row.

For example, if the input is:

name age  
alice 21  
ryan 30

we build:

- column 1 → `name alice ryan`
- column 2 → `age 21 30`

At the end, we print each built column on its own line.

This works efficiently because every field is visited exactly once.

---

### 🪜 Steps

1. **Read the file line by line**  
   Use `awk` to process each row of the file. Each row is automatically split into fields.

2. **Build transposed rows**  
   For each field index `i`, append the current field to `col[i]`.  
   - If it is the first row, initialize `col[i]` with that value.
   - Otherwise, append the new value with a space.

3. **Print the result**  
   After processing all rows, print `col[1]`, `col[2]`, ..., one per line.

---

## ✅ Constraints

- Each row has the same number of columns.
- Fields are separated by a single space character.
- The file is non-empty.
- The solution should work directly in shell.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(m × n)   |
| Space Complexity  | O(m × n)   |

Where:

- `m` = number of rows
- `n` = number of columns

---
