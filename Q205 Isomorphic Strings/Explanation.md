# 🧩 LeetCode Problem: Isomorphic Strings

- **Problem Link**: [Isomorphic Strings – LeetCode](https://leetcode.com/problems/isomorphic-strings/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/isomorphic-strings/solutions/)

---

## 🧠 Algorithm Explanation

To determine whether two strings are isomorphic, we need to ensure there is a **one-to-one mapping** between characters in `s` and characters in `t`.

This means:

- each character in `s` must always map to the same character in `t`
- two different characters in `s` cannot map to the same character in `t`

The optimal approach is to maintain **two hash maps**:

- one map from `s -> t`
- one map from `t -> s`

This allows us to verify the mapping in both directions while scanning the strings once.

This algorithm is used because it checks consistency efficiently in **linear time**, which is ideal given the string length can be up to `5 * 10^4`.

---

### 🪜 Steps

1. **Initialize two maps**:
   - one to store the mapping from characters in `s` to characters in `t`
   - one to store the reverse mapping from `t` to `s`

2. **Traverse both strings at the same time**:
   - for each pair of characters `(s[i], t[i])`, check whether an existing mapping conflicts

3. **Return the result**:
   - if any conflict is found, return `false`
   - otherwise, after processing all characters, return `true`

---

## ✅ Constraints

- `1 <= s.length <= 5 * 10^4`
- `t.length == s.length`
- `s` and `t` consist of any valid ASCII character

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
