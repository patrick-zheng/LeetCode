SELECT
    request_at AS Day,
    ROUND(AVG(status <> 'completed'), 2) AS `Cancellation Rate`
FROM Trips t
INNER JOIN Users c ON t.client_id = c.users_id AND c.banned = 'No'
INNER JOIN Users d ON t.driver_id = d.users_id AND d.banned = 'No'
WHERE request_at BETWEEN '2013-10-01' AND '2013-10-03'
GROUP BY request_at;
