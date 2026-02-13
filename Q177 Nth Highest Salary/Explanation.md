# 🧩 LeetCode Problem: 177. Nth Highest Salary  

- **Problem Link**: <https://leetcode.com/problems/nth-highest-salary/>  
- **Solution Link**: <https://leetcode.com/problems/nth-highest-salary/solutions/>  

---

## 🧠 Algorithm Explanation

The goal is to return the **N-th highest distinct salary** from the `Employee` table.

Key requirements:

- Salaries must be **distinct**
- If fewer than **N distinct salaries** exist → return `NULL`

### Why this approach?

We use:

- `DISTINCT` → to remove duplicate salary values  
- `ORDER BY salary DESC` → to rank salaries from highest to lowest  
- `LIMIT 1 OFFSET N-1` → to select the N-th element in the sorted list  

Since MySQL does not allow expressions directly inside `OFFSET`, we compute `N - 1` into a variable first.

---

## 🪜 Steps

1. **Remove duplicates**  
   Use `SELECT DISTINCT salary FROM Employee`.

2. **Sort in descending order**  
   Apply `ORDER BY salary DESC` to rank salaries.

3. **Select the N-th salary**  
   Use `LIMIT 1 OFFSET (N - 1)` to retrieve the correct row.  
   If it does not exist, MySQL automatically returns `NULL`.

---

## ✅ Constraints

- `1 <= N`
- Salary values are integers
- Must return the N-th **distinct** salary
- Return `NULL` if fewer than N distinct salaries exist

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(M log M) |
| Space Complexity  | O(M)       |

Where:

- `M` = number of distinct salaries  

Sorting dominates the time complexity.
