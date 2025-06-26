# 🧩 LeetCode Problem: Letter Combinations of a Phone Number

- **Problem Link**: [Letter Combinations of a Phone Number – LeetCode](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/)

---

## 🧠 Algorithm Explanation

The algorithm uses a backtracking approach to explore all possible letter combinations that the input string of digits could represent based on the phone number mapping. Each digit from 2 to 9 corresponds to a set of letters, and we generate all combinations by recursively selecting one letter from each digit’s possible set.

---

### 🪜 Steps

1. **Step 1**: Initialize a dictionary `digit_to_letters` that maps each digit to its corresponding letters on the phone keypad.

2. **Step 2**: Create a backtracking function that recursively explores combinations by appending letters for each digit.

3. **Step 3**: If the current path reaches the length of the input digits, store the valid combination in the result list. If not, continue exploring the next possible letters.

---

## ✅ Constraints

- The input string `digits` is non-empty and consists of digits from 2 to 9 only.
- The result should be in any order.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(4^n)     |
| Space Complexity  | O(n)       |
