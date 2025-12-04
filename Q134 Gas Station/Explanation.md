# 🧩 LeetCode Problem: Gas Station

- **Problem Link**: [Gas Station – LeetCode](https://leetcode.com/problems/gas-station/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/gas-station/solutions/)

---

## 🧠 Algorithm Explanation

We want to find a starting gas station index such that we can complete a full circle, given:

- `gas[i]`: gas we gain at station `i`
- `cost[i]`: gas required to go from station `i` to station `i + 1`

Key ideas:

1. **Feasibility check**:  
   If the total gas over all stations is less than the total cost, it is **impossible** to complete the circuit from any starting station.  
   Formally, if  
   \[
   \sum gas[i] < \sum cost[i]
   \]  
   then return `-1`.

2. **Greedy starting index**:
   We simulate traveling from left to right, keeping track of the current `tank` (gas in the car).  
   At each station `i`, we update:
   \[
   \text{tank} += gas[i] - cost[i]
   \]
   If at some point `tank < 0`, it means **we cannot reach station `i + 1` from the current start**.  
   Moreover, we also know that **none** of the stations between the current `start` and `i` can be a valid starting point, so we:
   - Set `start = i + 1`
   - Reset `tank = 0`

3. At the end:
   - If total gas ≥ total cost, `start` is the unique valid starting index.
   - Otherwise, return `-1`.

This greedy approach works because whenever we fail at some station `i`, every index between the previous `start` and `i` has an even worse deficit by that point, so none of them can be a valid start.

---

### 🪜 Steps

1. **Initialize variables**:
   - `total = 0` → cumulative net gas across all stations.
   - `tank = 0` → current gas in tank during the pass.
   - `start = 0` → candidate starting station index.

2. **Single pass through all stations**:
   - For each index `i` from `0` to `n - 1`:
     - Compute `diff = gas[i] - cost[i]`
     - Update `total += diff`
     - Update `tank += diff`
     - If `tank < 0`:
       - This means we cannot reach station `i + 1` from the current `start`.
       - Set `start = i + 1`
       - Reset `tank = 0`

3. **Final decision**:
   - If `total < 0` → return `-1` (not enough gas overall).
   - Otherwise → return `start` (the unique valid starting index).

---

## ✅ Constraints

- `n == gas.length == cost.length`
- `1 <= n <= 10^5`
- `0 <= gas[i] <= 10^4`
- `0 <= cost[i] <= 10^4`
- If a solution exists, it is guaranteed to be unique.

---

## ⏱ Time and Space Complexity

| Metric           | Complexity |
|------------------|------------|
| Time Complexity  | `O(n)`     |
| Space Complexity | `O(1)`     |

---
