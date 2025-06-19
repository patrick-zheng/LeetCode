# LeetCode Problem: Longest Substring Without Repeating Characters

- **Problem Link**: [Longest Substring Without Repeating Characters – LeetCode](https://leetcode.com/problems/longest-substring-without-repeating-characters/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/)

---

## 🧠 Algorithm Explanation

This problem uses the **sliding window** technique to maintain a dynamic range of characters that form a substring without duplicates.

- Use two pointers, `left` and `right`, to track the current window.
- Use a hash map to record the most recent index of each character.
- If a duplicate character is found, move the `left` pointer to the position just after its last occurrence.
- Continuously update the maximum window size (`max_length`) as the window grows.

This approach ensures that every character is processed at most twice, making it efficient even for long strings.

---

## ⏱ Time and Space Complexity

- **Time Complexity**: O(n)
  Each character is visited at most twice: once when `right` expands and possibly again when `left` is updated.

- **Space Complexity**: O(n)
  The hash map may store up to `n` entries in the worst case (e.g., all unique characters).

---
