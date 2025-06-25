# 🧩 LeetCode Problem: Integer to Roman

- **Problem Link**: [Integer to Roman – LeetCode](https://leetcode.com/problems/integer-to-roman/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/integer-to-roman/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires converting an integer into a Roman numeral, following a set of predefined rules. The algorithm uses a greedy approach, iterating through a list of Roman numeral values in descending order. For each numeral, it repeatedly appends the corresponding Roman numeral symbol to the result string, reducing the number accordingly.

This approach is efficient because it minimizes the number of operations required to convert the integer to a Roman numeral by directly checking the largest possible values first.

---

### 🪜 Steps

1. **Step 1**: Initialize a list of tuples with the Roman numeral values and their corresponding symbols, ordered from the largest value to the smallest.

2. **Step 2**: For each tuple in the list, divide the number by the Roman numeral value. The quotient tells how many times the symbol can be added to the result. Subtract the appropriate value from the number.

3. **Step 3**: Repeat the process for each numeral, reducing the number until it reaches zero.

4. **Step 4**: Return the constructed Roman numeral as a string.

---

## ✅ Constraints

- 1 <= num <= 3999

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(1)       |
| Space Complexity  | O(1)       |

---
