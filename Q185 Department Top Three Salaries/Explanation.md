# 🧩 LeetCode Problem: Department Top Three Salaries

- **Problem Link**: <https://leetcode.com/problems/department-top-three-salaries/>
- **Solution Link**: <https://leetcode.com/problems/department-top-three-salaries/solutions/>

---

## 🧠 Algorithm Explanation

We are given two tables:

- Employee(id, name, salary, departmentId)
- Department(id, name)

A **high earner** is an employee whose salary is among the **top three unique salaries within their department**.

### Key Points

- Ranking is done **per department**
- Only **distinct salaries** count toward ranking
- Multiple employees with the same salary must all be included
- Output order does not matter

The most optimal approach uses the window function `DENSE_RANK()`:

- `PARTITION BY departmentId` → rank per department
- `ORDER BY salary DESC` → highest salaries first
- `DENSE_RANK()` → ensures duplicate salaries share the same rank without gaps

We then filter for `rank <= 3`.

---

## 🪜 Steps

1. Rank salaries within each department using `DENSE_RANK()`.
2. Partition by `departmentId`.
3. Order salaries descending.
4. Filter rows where rank ≤ 3.
5. Join with Department table to get department names.

---

## ✅ Optimal SQL Solution

    SELECT 
        d.name AS Department,
        e.name AS Employee,
        e.salary AS Salary
    FROM (
        SELECT 
            e.*,
            DENSE_RANK() OVER (
                PARTITION BY departmentId
                ORDER BY salary DESC
            ) AS rnk
        FROM Employee e
    ) e
    JOIN Department d 
        ON e.departmentId = d.id
    WHERE e.rnk <= 3;

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(N log N) |
| Space Complexity  | O(N)       |

Where N is the number of employees.
