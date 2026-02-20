# 🧩 LeetCode Problem: Duplicate Emails

- **Problem Link**: [Duplicate Emails – LeetCode](https://leetcode.com/problems/duplicate-emails/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/duplicate-emails/solutions/)

---

## 🧠 Algorithm Explanation

To find duplicate emails in the `Person` table, we use SQL aggregation.

Since we need to return emails that appear more than once, the most efficient approach is:

- Group rows by `email`
- Count how many times each email appears
- Filter groups where the count is greater than 1

This works because `GROUP BY` clusters identical values together, and `HAVING` allows filtering on aggregated results.

This solution is optimal because it performs a single aggregation pass over the table.

---

### 🪜 Steps

1. **Group by email**
   - Use `GROUP BY email` to group identical emails together.

2. **Count occurrences**
   - Use `COUNT(*)` to count how many rows exist in each group.

3. **Filter duplicates**
   - Use `HAVING COUNT(*) > 1` to return only emails that appear more than once.

---

## ✅ Constraints

- `id` is the primary key.
- `email` is NOT NULL.
- Emails contain only lowercase letters.
- Return result in any order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

**Explanation:**

- Time Complexity: The table is scanned once and grouped — linear in the number of rows.
- Space Complexity: The grouping operation may store up to `n` unique emails in memory in the worst case.
