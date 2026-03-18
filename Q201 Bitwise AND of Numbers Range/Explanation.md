# 🧩 LeetCode Problem: 201. Bitwise AND of Numbers Range

- **Problem Link**: [201. Bitwise AND of Numbers Range – LeetCode](https://leetcode.com/problems/bitwise-and-of-numbers-range/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks for the bitwise AND of every number in the inclusive range `[left, right]`.

The key observation is that any bit that changes between `left` and `right` will become `0` in the final answer, because somewhere in that range there will be both a `0` and a `1` at that position. So the result is simply the common binary prefix shared by `left` and `right`.

To find that common prefix, we keep right-shifting both numbers until they become equal. This removes the differing lower bits. Then we shift the common value back left by the same number of shifts.

---

### 🪜 Steps

1. **Step 1**: Repeatedly right-shift `left` and `right` until they become equal.
2. **Step 2**: Count how many shifts were needed while removing the differing lower bits.
3. **SStep 3**: Left-shift the common prefix back by that count to get the final answer.

---

## ✅ Constraints

- `0 <= left <= right <= 2^31 - 1`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity    |
|-------------------|---------------|
| Time Complexity   | O(log(right)) |
| Space Complexity  | O(1)          |

---
