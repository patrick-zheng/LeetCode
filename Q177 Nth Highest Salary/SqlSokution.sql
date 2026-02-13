CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  DECLARE off INT DEFAULT 0;
  SET off = N - 1;

  RETURN (
    SELECT salary
    FROM (
      SELECT DISTINCT salary
      FROM Employee
    ) s
    ORDER BY salary DESC
    LIMIT 1 OFFSET off
  );
END
    