SELECT DISTINCT num AS ConsecutiveNums
FROM (
    SELECT
        num,
        LAG(num, 1) OVER (ORDER BY id) AS p,
        LAG(num, 2) OVER (ORDER BY id) AS pp
    FROM Logs
) t
WHERE num = p AND num = pp;
    