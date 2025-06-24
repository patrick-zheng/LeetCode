# 🧩 LeetCode Problem: Palindrome Number

- **Problem Link**: [Palindrome Number – LeetCode](https://leetcode.com/problems/palindrome-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/palindrome-number/solutions/)

---

## 🧠 Algorithm Explanation

The algorithm checks whether an integer is a palindrome by reversing the entire number and comparing it to the original value. If both are equal, the number is a palindrome.

This approach is straightforward, avoids converting the number to a string or using extra data structures, and works entirely with arithmetic, which is efficient for small to moderate-sized integers.

---

### 🪜 Steps

1. **Handle Negatives**: Return `False` immediately if the number is negative (since `-121` ≠ `121-`).
2. **Reverse the Number**: Extract each digit from the end using `x % 10` and build a reversed number with `rev = rev * 10 + digit`.
3. **Compare**: Return `True` if the reversed number matches the original.

---

## Constraints

- `-2³¹ <= x <= 2³¹ - 1`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(log₁₀(n)) |
| Space Complexity  | O(1)        |

Where `n` is the input number. Time is proportional to the number of digits.

---
