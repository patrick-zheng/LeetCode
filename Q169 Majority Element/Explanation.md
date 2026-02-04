# 🧩 LeetCode Problem: Majority Element

- **Problem Link**: [Majority Element – LeetCode](https://leetcode.com/problems/majority-element/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/majority-element/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks us to find the element that appears **more than ⌊n / 2⌋ times** in an array of size `n`.  
A key detail is that the problem **guarantees the majority element always exists**.

The most optimal solution uses the **Boyer–Moore Voting Algorithm**.  
The intuition is that the majority element appears more times than all other elements combined, so if we "cancel out" different elements against each other, the majority element will always remain.

This allows us to solve the problem in **linear time** and **constant space**, which is optimal.

---

### 🪜 Steps

1. **Initialize a candidate and a counter**
   - Start with `count = 0` and no candidate.

2. **Traverse the array**
   - If `count == 0`, set the current element as the new candidate.
   - If the current element equals the candidate, increment `count`.
   - Otherwise, decrement `count`.

3. **Return the candidate**
   - Since the majority element is guaranteed to exist, the final candidate is the answer.

---

## ✅ Constraints

- `1 <= nums.length <= 5 * 10⁴`
- `-10⁹ <= nums[i] <= 10⁹`
- The majority element **always exists** in the array.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(1)**   |

---
