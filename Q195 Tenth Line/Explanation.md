# 🧩 LeetCode Problem: 195. Tenth Line

- **Problem Link**: [195. Tenth Line – LeetCode](https://leetcode.com/problems/tenth-line/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/tenth-line/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks us to print the 10th line of the file `file.txt`.

The optimal and cleanest solution is to use `awk`:

`awk 'NR==10{print; exit}' file.txt`

Why this works:

- `NR` is the current line number in `awk`
- when `NR == 10`, we print that line
- `exit` stops the program immediately after printing, so it does not keep scanning the rest of the file

This makes the solution simple, efficient, and easy to explain.

---

### 🪜 Steps

1. **Step 1**: Read the file line by line.
2. **Step 2**: Check if the current line number is 10.
3. **Step 3**: Print the 10th line and stop immediately.

---

## ✅ Constraints

- The file may contain fewer than 10 lines.
- If the file has fewer than 10 lines, output nothing.
- The solution must be written using shell commands.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |    O(n)    |
| Space Complexity  |    O(1)    |

---
