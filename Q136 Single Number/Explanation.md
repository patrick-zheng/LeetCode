# 🧩 LeetCode Problem: Single Number

- **Problem Link**: [Single Number – LeetCode](https://leetcode.com/problems/single-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/single-number/solutions/)

---

## 🧠 Algorithm Explanation

We use the **bitwise XOR** operation to find the single number.

Key XOR properties:

- `a ^ a = 0` → a number XORed with itself cancels out  
- `a ^ 0 = a`  
- XOR is **commutative** and **associative**, so the order does not matter  

Since every element appears **exactly twice** except for one, if we XOR all elements together, all pairs will cancel out and we will be left with the number that appears only once.

This satisfies:

- **Linear time**: one pass over the array  
- **Constant space**: only one accumulator variable is used  

---

### 🪜 Steps

1. **Initialize** a variable `res = 0`.
2. **Iterate** through each number `num` in `nums` and update `res ^= num`.
3. **Return** `res` at the end; it will hold the single number.

---

## ✅ Constraints

- `1 <= nums.length <= 3 * 10^4`
- `-3 * 10^4 <= nums[i] <= 3 * 10^4`
- Every element appears **twice** except for one, which appears **exactly once**.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
