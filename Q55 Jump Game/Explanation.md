# 🧩 LeetCode Problem: Jump Game

- **Problem Link**: [Jump Game – LeetCode](https://leetcode.com/problems/jump-game/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/jump-game/solutions/)

---

## 🧠 Algorithm Explanation

We need to determine if we can reach the last index of the array starting from the first index, where each element represents the maximum jump length from that position.

A **greedy algorithm** works best here. Instead of exploring all possible paths (which would be exponential), we only need to track whether the current index can reach a "goal."  
There are two common approaches:

- **Forward Greedy**: Track the farthest index reachable at each step.
- **Backward Greedy**: Track the leftmost index that can reach the goal, updating the goal until it reaches index 0.

Both achieve **O(n) time** and **O(1) space**, which is optimal.

---

### 🪜 Steps

1. **Initialize the Goal**: Set the goal as the last index of the array.
2. **Traverse Backwards**: Iterate from the second-to-last index down to the first.
3. **Check Reachability**: If the current index can jump to or beyond the goal, update the goal to this index.
4. **Final Check**: If the goal becomes index `0`, then we can reach the end.

---

## ✅ Constraints

- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 10^5`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
