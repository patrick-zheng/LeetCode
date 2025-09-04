# 🧩 LeetCode Problem: Text Justification

- **Problem Link**: [Text Justification – LeetCode](https://leetcode.com/problems/text-justification/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/text-justification/solutions/)

---

## 🧠 Algorithm Explanation

Use a **greedy line builder**:

- Accumulate words into the current line while the total length (letters + required single spaces between words) does **not** exceed `maxWidth`.
- When adding the next word would overflow:
  - If the line has **one word** or it’s the **last line**, **left-justify**: join with single spaces and pad the end with spaces to reach `maxWidth`.
  - Otherwise, **fully justify**: compute the number of spaces to insert, split them evenly across the gaps, and distribute any remainder to the **leftmost** gaps.
- Continue until all words are processed; format the final line as left-justified.

This works because each line’s spacing depends only on the words in that line, so a single pass with local formatting per line suffices.

---

### 🪜 Steps

1. **Build lines greedily**: Track `curr_line` (list of words) and `num_of_letters` (sum of their lengths). If adding the next word would exceed `maxWidth`, finalize the current line.
2. **Distribute spaces**:
   - Let `gaps = len(curr_line) - 1` and `spaces = maxWidth - num_of_letters`.
   - If `gaps == 0` **or** it’s the **last line**: join with single spaces and right-pad using `ljust(maxWidth)`.
   - Else: each gap gets `spaces // gaps` base spaces; the first `spaces % gaps` gaps get one extra space.
3. **SStep 3**: **Finalize & continue**: Append the justified string to results, reset accumulators, and continue. After the loop, left-justify the remaining (last) line.

---

## ✅ Constraints

- `1 ≤ maxWidth`
- Each `words[i]` is a non-empty string without internal spaces.
- `maxWidth` ≥ length of the longest word.
- Every output line must be exactly `maxWidth` characters.
- All lines except the last must be fully justified; the last line is left-justified.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(T), where **T** is the total number of characters in `words` (plus inserted spaces); effectively linear. |
| Space Complexity  | O(1) auxiliary (excluding the output list of lines). |

---
