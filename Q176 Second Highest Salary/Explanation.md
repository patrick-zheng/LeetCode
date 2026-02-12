# 🧩 LeetCode Problem: Second Highest Salary

- **Problem Link**: [Second Highest Salary – LeetCode](https://leetcode.com/problems/second-highest-salary/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/second-highest-salary/solutions/)

---

## 🧠 Algorithm Explanation

We are given an `Employee` table containing employee salaries. The task is to return the **second highest distinct salary**.

Important details:

- Salaries must be **distinct**.
- If there is no second highest salary, return `NULL`.

### Approach

1. Use `DISTINCT` to remove duplicate salaries.
2. Sort salaries in descending order.
3. Skip the highest salary.
4. Return the next value if it exists.

If fewer than two distinct salaries exist, the result will naturally be `NULL`.

---

## 🪜 Steps

1. Select distinct salaries from `Employee`.
2. Order them in descending order.
3. Use `LIMIT 1 OFFSET 1` to get the second highest salary.
4. Return the result as `SecondHighestSalary`.

---

## ✅ Constraints

- `1 <= Employee.id <= 10^4`
- `-10^9 <= Employee.salary <= 10^9`
- Salaries may contain duplicates.
- Return `NULL` if no second highest salary exists.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n log n) |
| Space Complexity  | O(n)       |

Sorting distinct salaries dominates the time complexity, and storing them requires additional space.
