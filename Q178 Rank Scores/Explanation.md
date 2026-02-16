# 🧩 LeetCode Problem: Rank Scores

- **Problem Link**: [Rank Scores – LeetCode](https://leetcode.com/problems/rank-scores/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rank-scores/solutions/)

---

## 🧠 Algorithm Explanation

We need to assign a **dense rank** to each score:

- Rank scores from **highest to lowest**
- **Tied** scores share the **same** rank
- After a tie, the next rank is the **next integer** (no gaps)

To do this in MySQL, we use a correlated subquery that computes the rank of each `s1.score` by counting how many **distinct** scores are **greater than or equal** to it. That count directly matches the dense rank.

We also wrap the alias **`rank`** in backticks because `RANK` is a reserved keyword in MySQL.

---

### 🪜 Steps

1. **Step 1**: For each row `s1`, look at its `score`.
2. **Step 2**: Count how many **distinct** scores `s2.score` satisfy `s2.score >= s1.score`.
3. **Step 3**: Use that count as the dense rank, and order the final output by `score` descending.

---

## ✅ Constraints

- `score` values can repeat (ties must share the same rank).
- Output must use **dense ranking** (no gaps after ties).
- Result must be ordered by `score` in **descending** order.
- `id` is unique but not needed for ranking output.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                          |
|-------------------|------------                         |
| Time Complexity   | O(n²) (correlated subquery per row) |
| Space Complexity  | O(1)                                |

---
