# 🧩 LeetCode Problem: Consecutive Numbers

- **Problem Link**: [Consecutive Numbers – LeetCode](https://leetcode.com/problems/consecutive-numbers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/consecutive-numbers/solutions/)

---

## 🧠 Algorithm Explanation

We are given a table `Logs(id, num)` where:

- `id` is the primary key (auto-incremented).
- `num` is the value recorded in order.

We must find all numbers that appear **at least three times consecutively**.

Because `id` determines strict ordering, we can compare each row with its previous two rows.  
If the current value equals both previous values, then it forms a sequence of at least three consecutive identical numbers.

Using SQL window functions (`LAG`) is optimal because:

- It performs a single logical pass over the table.
- It avoids multiple self-joins.
- It cleanly compares adjacent rows.

---

### 🪜 Steps

1. Order rows by `id`.
2. Use `LAG(num, 1)` to get the previous value.
3. Use `LAG(num, 2)` to get the value two rows back.
4. Filter rows where:
   - `num = previous value`
   - `num = value two rows back`
5. Use `DISTINCT` to avoid duplicates for sequences longer than three.

---

## ✅ Constraints

- `id` is auto-incremented.
- Consecutiveness is determined by `id` ordering.
- Result order does not matter.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

---
