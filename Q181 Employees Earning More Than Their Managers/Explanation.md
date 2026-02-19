# 🧩 LeetCode Problem: Employees Earning More Than Their Managers

- **Problem Link**: <https://leetcode.com/problems/employees-earning-more-than-their-managers/>
- **Solution Link**: <https://leetcode.com/problems/employees-earning-more-than-their-managers/solutions/>

---

## 🧠 Algorithm Explanation

This problem asks us to find employees whose salary is strictly greater than their manager’s salary.

Since both employees and managers are stored in the same `Employee` table, we use a **self-join**. A self-join allows us to treat the same table as two different instances:

- `e` → represents the employee  
- `m` → represents the manager  

We join the table using:

`e.managerId = m.id`

This connects each employee to their corresponding manager.  
We then filter rows where the employee’s salary is greater than the manager’s salary:

`e.salary > m.salary`

This approach is optimal because:

- It avoids nested subqueries.
- It directly compares related rows.
- It efficiently uses the primary key index on `id`.

---

## 🪜 Steps

1. Perform a self-join on the `Employee` table using `managerId`.
2. Filter rows where the employee's salary is greater than the manager's salary.
3. Select and return the employee’s name.

---

## ✅ Constraints

- `id` is the primary key.
- `managerId` may be `NULL` (for top-level managers).
- Salaries are integers.
- The result can be returned in any order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

### Explanation

- The join operates in linear time relative to the number of rows.
- No extra memory structures are created.
- Only filtered rows are returned.
