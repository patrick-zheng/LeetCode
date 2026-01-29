# 🧩 LeetCode Problem: Compare Version Numbers

- **Problem Link**: [Compare Version Numbers – LeetCode](https://leetcode.com/problems/compare-version-numbers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/compare-version-numbers/solutions/)

---

## 🧠 Algorithm Explanation

We compare the two version strings **revision-by-revision** from left to right.  
Each revision is separated by `.` and is treated as an **integer**, meaning leading zeros do not affect the comparison.

If one version has fewer revisions, the missing revisions are treated as `0`.  
The first revision where the values differ determines the result.  
If all revisions are equal, the versions are considered equal.

This approach directly follows the problem definition and is efficient because it only requires a single linear pass.

---

### 🪜 Steps

1. **Split** both version strings using `.` as the delimiter.
2. **Iterate** from index `0` to `max(length(version1), length(version2)) - 1`.
3. For each index:
   - Convert the revision to an integer if it exists.
   - If it does not exist, use `0` as the value.
   - Compare the two integers:
     - If left < right → return `-1`
     - If left > right → return `1`
4. If no differences are found after the loop, **return `0`**.

---

## ✅ Constraints

- `1 <= version1.length, version2.length <= 500`
- Version strings contain only digits (`0–9`) and dots (`.`)
- Revision values are valid integers in the range `[0, 2^31 - 1]`
- Leading zeros are allowed but ignored during comparison

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(n)**   |

Where `n` is the total number of revisions processed.

---
