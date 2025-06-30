# 🧩 LeetCode Problem: **Problem Name**

- **Problem Link**: [Problem Name – LeetCode](https://leetcode.com/problems/problem-name/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/problem-name/solutions/)

---

## 🧠 Algorithm Explanation

The solution utilizes a backtracking approach to explore all possible combinations of parentheses, ensuring that each generated string satisfies the conditions of well-formed parentheses. This algorithm is efficient because it directly builds valid solutions, avoiding the need to check for validity after generating each string.

---

### 🪜 Steps

1. **Step 1**: Initialize a recursive function with parameters for the current string, the count of open and close parentheses, and the target number `n` (half of the total parentheses).

2. **Step 2**: Base case – If the length of the current string is `2 * n`, append the string to the result list since it represents a valid well-formed parentheses combination.

3. **Step 3**: If the number of open parentheses is less than `n`, add an open parenthesis and recursively call the function to continue building the string.

4. **Step 4**: If the number of close parentheses is less than the number of open parentheses, add a closing parenthesis and recursively call the function.

5. **Step 5**: Return the result list after the recursive exploration finishes.

---

## ✅ Constraints

- `1 <= n <= 8`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(4^n / sqrt(n)) |
| Space Complexity  | O(n)       |

---
