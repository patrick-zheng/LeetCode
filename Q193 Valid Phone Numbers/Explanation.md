# 🧩 LeetCode Problem: Valid Phone Numbers

- **Problem Link**: [Valid Phone Numbers – LeetCode](https://leetcode.com/problems/valid-phone-numbers/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-phone-numbers/solutions/)

---

## 🧠 Algorithm Explanation

The task is to print only the valid phone numbers from `file.txt`.  
A valid phone number must match **exactly one of two formats**:

1. `xxx-xxx-xxxx`
2. `(xxx) xxx-xxxx`

Where `x` represents a digit from `0–9`.

Since the input file contains one phone number per line, the most efficient solution is to **filter lines using a regular expression**. We use the `grep` command with **extended regular expressions (`-E`)**.

The regex ensures:

- The entire line matches the pattern (`^` start anchor, `$` end anchor)
- The number matches **either** of the two valid formats using `|`
- Digits are enforced with `[0-9]`
- Exact counts of digits are enforced with `{}`

### Bash One-Liner Solution

```bash
grep -E '^([0-9]{3}-[0-9]{3}-[0-9]{4}|\([0-9]{3}\) [0-9]{3}-[0-9]{4})$' file.txt
```

Explanation of the regex:

- `^` → start of the line  
- `[0-9]{3}` → exactly 3 digits  
- `-` → dash separator  
- `\(` and `\)` → literal parentheses  
- space → required space between area code and number  
- `|` → logical OR between the two valid formats  
- `$` → end of the line  

This ensures only lines that exactly match the valid phone number formats are printed.

---

### 🪜 Steps

1. **Read each line from `file.txt`** using `grep`.
2. **Enable extended regex** with `-E`.
3. **Match either valid phone number format** using a combined regex pattern.
4. **Print only the matching lines**, which correspond to valid phone numbers.

---

## ✅ Constraints

- A phone number must match one of the following formats:
  - `xxx-xxx-xxxx`
  - `(xxx) xxx-xxxx`
- `x` represents a digit (`0–9`)
- No leading or trailing spaces exist in each line
- Each line contains exactly one candidate phone number

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   |    O(n)    |
| Space Complexity  |    O(1)    |

Where **n** is the number of lines in the file. The command scans the file once and does not require extra memory.

---
