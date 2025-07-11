# 🧩 LeetCode Problem: Substring with Concatenation of All Words

- **Problem Link**: [Substring with Concatenation of All Words – LeetCode](https://leetcode.com/problems/substring-with-concatenation-of-all-words/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/substring-with-concatenation-of-all-words/solutions/)

---

## 🧠 Algorithm Explanation

To efficiently find all starting indices of substrings in `s` that are the concatenation of each word in `words` exactly once and without any intervening characters, we use a **sliding window** approach with **hash map (frequency count)** comparison.

Each substring of size `totalLen = len(words) * wordLen` is analyzed. We extract word-sized chunks within this window and count their frequencies. If the frequency map matches the expected word count, we record the starting index.

This avoids generating all permutations, reducing the complexity from factorial to linear scanning.

---

### 🪜 Steps

1. **Preprocessing**:  
   - Calculate `wordLen` as the length of a single word (all words are the same length).  
   - Compute `totalLen = wordLen * len(words)`  
   - Create a `wordCount` dictionary for the frequency of each word.

2. **Offset Iteration**:  
   Loop from `i = 0` to `wordLen - 1` to handle different alignments of substrings.

3. **Sliding Window**:  
   - Move the `right` pointer in steps of `wordLen`.
   - Track the current window of words in a `windowCount` map.
   - If the word is invalid or the count exceeds the expected, reset or shrink the window.
   - If the window size matches `totalLen` and maps match, record the start index.

---

## ✅ Constraints

- `1 <= s.length <= 10^4`
- `1 <= words.length <= 5000`
- `1 <= words[i].length <= 30`
- All words in `words` are the same length.
- `s` and `words[i]` consist of lowercase English letters.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity   |
|-------------------|--------------|
| Time Complexity   | O(m * n * k) |
| Space Complexity  | O(n * k)     |

Where:  

- `m = len(s)`  
- `n = len(words)`  
- `k = len(words[0])`
