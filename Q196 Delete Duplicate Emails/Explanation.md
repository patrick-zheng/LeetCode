# 🧩 LeetCode Problem: 196. Delete Duplicate Emails

- **Problem Link**: [196. Delete Duplicate Emails – LeetCode](https://leetcode.com/problems/delete-duplicate-emails/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/delete-duplicate-emails/solutions/)

---

## 🧠 Algorithm Explanation

The goal is to delete duplicate emails while keeping only the row with the smallest `id` for each email.

The standard optimal SQL approach is to use a self-join:

- compare rows with the same `email`
- if one row has a larger `id` than another, delete it
- this ensures only the smallest `id` remains for each unique email

This is the correct approach because the problem requires a `DELETE` statement, not a `SELECT`.

---

### 🪜 Steps

1. **Join the table with itself** on the same `email`.
2. **Find duplicate rows** where one row has a larger `id`.
3. **Delete the row with the larger `id`** so only the smallest `id` remains.

---

## ✅ SQL Solution

```sql
DELETE p1
FROM Person p1, Person p2
WHERE p1.email = p2.email
  AND p1.id > p2.id;
```

---

## ✅ Why This Works

For every duplicated email, the query compares two matching rows.

If:

- `p1.email = p2.email`
- and `p1.id > p2.id`

then `p1` is the duplicate with the larger `id`, so it gets deleted.

That leaves only the row with the smallest `id` for each email.

---

## ✅ Constraints

- `id` is the primary key
- `email` contains lowercase letters only
- keep exactly one row per unique email
- the kept row must have the smallest `id`

---

## ⏱ Time and Space Complexity

| Metric           | Complexity |
|------------------|------------|
| Time Complexity  |    O(n²)   |
| Space Complexity |    O(1)    |

---
