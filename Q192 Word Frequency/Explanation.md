# 🧩 LeetCode Problem: Word Frequency

- **Problem Link**: [Word Frequency – LeetCode](https://leetcode.com/problems/word-frequency/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-frequency/solutions/)

---

## 🧠 Algorithm Explanation

The objective is to compute the frequency of each word in a text file (`words.txt`) and output the results sorted by **descending frequency**.

Since the problem specifically asks for a **one-line solution using Unix pipes**, we combine several Unix text-processing utilities. The idea is to transform the text so that each word appears on its own line, group identical words together, count them, sort by frequency, and finally format the output.

The full pipeline is:

```bash
tr -s '[:space:]' '\n' < words.txt | sort | uniq -c | sort -nr | awk '{print $2, $1}'
```

Explanation of each command:

- **`tr -s '[:space:]' '\n'`**  
  Converts whitespace into newline characters so that every word appears on its own line.  
  The `-s` option squeezes repeated whitespace into a single newline.

- **`sort`**  
  Sorts all words alphabetically so identical words appear next to each other.

- **`uniq -c`**  
  Counts consecutive identical words and outputs them as `count word`.

- **`sort -nr`**  
  Sorts the results numerically (`-n`) and in reverse order (`-r`) so the highest frequency appears first.

- **`awk '{print $2, $1}'`**  
  Reorders the columns because `uniq -c` outputs `count word`, but the required format is `word count`.

---

### 🪜 Steps

1. Convert whitespace-separated words into one word per line using `tr`.
2. Sort the words alphabetically so duplicates are grouped together.
3. Count occurrences of each word using `uniq -c`.
4. Sort the results numerically in descending order.
5. Swap the column order to print `word count`.

---

## ✅ Constraints

- `words.txt` contains only lowercase characters and whitespace.
- Each word consists only of lowercase letters.
- Words are separated by one or more whitespace characters.
- Word frequency counts are guaranteed to be unique.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n log n) |
| Space Complexity  | O(n)       |
