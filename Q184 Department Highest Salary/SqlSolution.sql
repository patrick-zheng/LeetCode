SELECT
  d.name AS Department,
  e.name AS Employee,
  e.salary AS Salary
FROM Department d
JOIN (
  SELECT departmentId, MAX(salary) AS max_salary
  FROM Employee
  GROUP BY departmentId
) m
  ON m.departmentId = d.id
JOIN Employee e
  ON e.departmentId = m.departmentId
 AND e.salary = m.max_salary;
    