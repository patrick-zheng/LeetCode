# 🧩 LeetCode Problem: Single Number II

- **Problem Link**: [Single Number II – LeetCode](https://leetcode.com/problems/single-number-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/single-number-ii/solutions/)

---

## 🧠 Algorithm Explanation

We’re given an integer array where **every element appears exactly three times except for one**, which appears once.  
We need to find that unique element in **O(n)** time and **O(1)** extra space.

We use a **bitwise finite state machine (FSM)** over all bits using two integer masks:

- `ones` – bits that have appeared **once so far (mod 3)**  
- `twos` – bits that have appeared **twice so far (mod 3)**  

For each number `x`, we update:

- `ones = (ones ^ x) & ~twos`  
- `twos = (twos ^ x) & ~ones`

Intuition:

- `ones ^ x` toggles bits where `x` has 1s (standard XOR).
- `& ~twos` removes from `ones` the bits that have now appeared twice.
- Similarly, `twos` tracks bits seen twice but clears those that moved back into `ones`.
- Since every non-unique element appears **three times**, each bit cycles:

  `0 → 1 → 2 → 0 (mod 3)`

After processing all elements, the bits that are **1 mod 3** are exactly those of the unique number, stored in `ones`.

---

### 🪜 Steps

1. **Initialize masks**
   - Set `ones = 0`, `twos = 0`.

2. **Iterate through the array**
   - For each element `x` in `nums`:
     - Update  
       `ones = (ones ^ x) & ~twos`  
       `twos = (twos ^ x) & ~ones`

3. **Return the result**
   - After the loop finishes, `ones` holds the unique element.
   - Return `ones`.

---

## ✅ Constraints

- `1 <= nums.length <= 3 * 10^4`
- `-2^31 <= nums[i] <= 2^31 - 1`
- Every element appears **three times** except for one that appears **exactly once**.
- Must run in **linear time** and use **constant extra space**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
