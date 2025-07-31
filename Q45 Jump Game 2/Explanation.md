# 🧩 LeetCode Problem: Jump Game II

- **Problem Link**: [Jump Game II – LeetCode](https://leetcode.com/problems/jump-game-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/jump-game-ii/solutions/)

---

## 🧠 Algorithm Explanation

We use a **greedy algorithm** to solve this problem in linear time. The goal is to always jump to the position that allows us to reach the farthest possible index with the fewest number of jumps.

We maintain two variables:

- `farthest`: the farthest index we can currently reach.
- `current_end`: the boundary of the current jump.

When our index `i` reaches `current_end`, it means we must make a jump. At that point, we increment the jump count and update `current_end` to `farthest`.

This guarantees the minimum number of jumps while only scanning the array once.

---

### 🪜 Steps

1. **Initialize** `jumps = 0`, `farthest = 0`, and `current_end = 0`.
2. **Iterate** through the array from index `0` to `n - 2` (we don't need to consider the last element).
3. **Update** `farthest` to be the maximum of `farthest` and `i + nums[i]`.
4. If `i == current_end`, increment `jumps` and update `current_end` to `farthest`.
5. **Return** `jumps` once the loop completes.

---

## ✅ Constraints

- `1 <= nums.length <= 10^4`
- `0 <= nums[i] <= 1000`
- It is guaranteed that you can reach the last index.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
