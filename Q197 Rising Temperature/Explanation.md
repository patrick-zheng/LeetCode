# 🧩 LeetCode Problem: 197. Rising Temperature

- **Problem Link**: [197. Rising Temperature – LeetCode](https://leetcode.com/problems/rising-temperature/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/rising-temperature/solutions/)

---

## 🧠 Algorithm Explanation

We need to return the `id` of each day where the temperature is higher than **the previous calendar day**.

The important detail is that this is **not** asking for the previous row by `id`. It specifically wants the row whose `recordDate` is exactly **1 day after** another row's `recordDate`.

The optimal SQL approach is to use a **self-join**:

- one copy of the table represents the **current day**
- another copy represents the **previous day**
- join them when their dates differ by exactly 1 day
- keep only the rows where the current day's temperature is greater than the previous day's temperature

This is the cleanest and standard solution because it directly models the condition in the problem.

---

### 🪜 Steps

1. **Create two aliases of the `Weather` table**  
   One alias is for the current day (`w1`) and the other is for the previous day (`w2`).

2. **Join rows that are exactly one day apart**  
   Match rows where `w1.recordDate = DATE_ADD(w2.recordDate, INTERVAL 1 DAY)`.

3. **Check whether temperature increased**  
   Keep only the rows where `w1.temperature > w2.temperature`.

4. **Return the current day's `id`**  
   The problem asks for the `id` of the day with the higher temperature.

---

## ✅ SQL Solution

    SELECT w1.id
    FROM Weather w1
    JOIN Weather w2
        ON w1.recordDate = DATE_ADD(w2.recordDate, INTERVAL 1 DAY)
    WHERE w1.temperature > w2.temperature;

---

## ✅ Constraints

- `id` is unique
- `recordDate` is unique
- We must compare with the **previous calendar day**
- Return the result in **any order**

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                |
|-------------------|---------------------------|
| Time Complexity   | O(n²) in the general case |
| Space Complexity  |      O(1) extra space     |

---
