# 🧩 LeetCode Problem: Department Highest Salary

- **Problem Link**: [Department Highest Salary – LeetCode](https://leetcode.com/problems/department-highest-salary/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/department-highest-salary/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to find employees who earn the highest salary in each department.

To achieve this efficiently:

- First, compute the **maximum salary per department** using `GROUP BY`.
- Then, join this result back to the `Employee` table to retrieve the employees whose salary equals that maximum.
- Finally, join with the `Department` table to get the department name.

This approach is optimal because:

- It scans the `Employee` table once for aggregation.
- It correctly handles **ties** (multiple employees with the same highest salary).
- It avoids correlated subqueries that may be less efficient.

---

### 🪜 Steps

1. Use `GROUP BY departmentId` and `MAX(salary)` to compute the highest salary in each department.
2. Join the aggregated result with the `Employee` table on matching `departmentId` and `salary = max_salary`.
3. Join with the `Department` table to return the department name.

---

## ✅ Constraints

- `Employee.id` and `Department.id` are primary keys.
- `Employee.departmentId` is a foreign key referencing `Department(id)`.
- Department names are NOT NULL.
- Multiple employees may share the highest salary in a department (ties must be included).
- Result can be returned in any order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |    O(N)    |
| Space Complexity  |    O(D)    |

Where:

- `N` = number of employees  
- `D` = number of departments  

The grouping step scans all employees once → **O(N)**.  
The temporary aggregated result stores at most one row per department → **O(D)**.
