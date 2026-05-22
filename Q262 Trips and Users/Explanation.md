# LeetCode Problem: Trips and Users

- **Problem Link**: [Trips and Users – LeetCode](https://leetcode.com/problems/trips-and-users/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/trips-and-users/solutions/)

---

## Algorithm

For each day in `["2013-10-01", "2013-10-03"]`, compute:

\[
\text{Cancellation Rate} = \frac{C}{N}
\]

where \(C\) is the count of cancelled trips and \(N\) is the total trip count,
each restricted to trips whose **client** and **driver** are both unbanned.

A trip is **cancelled** when `status` is not `'completed'` (either
`cancelled_by_driver` or `cancelled_by_client`). A user is **unbanned** when
`banned = 'No'`.

### Approach

1. **Filter trips** to the required date range.
2. **Keep only unbanned participants** by joining `Trips` to `Users` twice:
   once for `client_id`, once for `driver_id`, requiring `banned = 'No'` on
   both joins.
3. **Group by** `request_at` (the output column `Day`).
4. For each group, count cancelled rows with
   `SUM(status <> 'completed')` or equivalently `AVG(status <> 'completed')`
   (MySQL treats the comparison as `0`/`1`), divide by `COUNT(*)`, and
   **`ROUND(..., 2)`**.

`INNER JOIN` on unbanned users is preferred over `NOT IN (SELECT ... WHERE
banned = 'Yes')`: it is clear, index-friendly, and avoids subtle `NULL` issues
with `NOT IN`.

Only days with at least one qualifying trip appear because `GROUP BY` has no
matching rows for empty groups.

---

## Constraints

- `Trips.id` is unique; `request_at` is a date string in the example range.
- `status` is one of `'completed'`, `'cancelled_by_driver'`,
  `'cancelled_by_client'`.
- Both **client** and **driver** must be unbanned for a trip to count.
- Report days from `"2013-10-01"` through `"2013-10-03"` inclusive.
- Round `Cancellation Rate` to **two** decimal places.
- Return rows in any order.

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(T)** — one pass over trips in the date window after joins to `Users`        |
| Space Complexity   | **O(1)** extra — aggregate per day; at most three output rows for this window   |

---
