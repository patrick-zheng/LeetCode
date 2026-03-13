# 🧩 LeetCode Problem: House Robber

- **Problem Link**: [House Robber – LeetCode](https://leetcode.com/problems/house-robber/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/house-robber/solutions/)

---

## 🧠 Algorithm Explanation

This problem is a classic **dynamic programming** problem.

At each house, you only have **two choices**:

1. **Do not rob the current house**  
   Then your total remains the best amount from the previous house.

2. **Rob the current house**  
   Then you cannot rob the previous house, so you add the current house's money to the best amount from two houses back.

This gives the recurrence:

\[
dp[i] = \max(dp[i-1], dp[i-2] + nums[i])
\]

Instead of storing the whole `dp` array, we notice that each state only depends on the previous two states. So we can optimize space by keeping only:

- `prev1` = best result up to previous house
- `prev2` = best result up to the house before that

This makes the solution both efficient and optimal.

---

### 🪜 Steps

1. **Initialize two variables**
   - `prev2 = 0` → best amount up to house `i-2`
   - `prev1 = 0` → best amount up to house `i-1`

2. **Iterate through each house**
   - For each `money` in `nums`, compute:
     - `current = max(prev1, prev2 + money)`

3. **Shift the variables forward**
   - Set `prev2 = prev1`
   - Set `prev1 = current`

4. **Return the final answer**
   - `prev1` will contain the maximum amount that can be robbed without alerting the police

---

## ✅ Constraints

- `1 <= nums.length <= 100`
- `0 <= nums[i] <= 400`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
