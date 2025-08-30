# 🧩 LeetCode Problem: Valid Number

- **Problem Link**: [Valid Number – LeetCode](https://leetcode.com/problems/valid-number/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/valid-number/solutions/)

---

## 🧠 Algorithm Explanation

We need to determine if a string represents a **valid number**.  
A valid number can be:

- An **integer** (with optional sign, e.g. `+3`, `-42`).
- A **decimal** (with optional sign, e.g. `3.14`, `-0.1`, `4.`, `-.9`).
- Either of the above followed by an **exponent** (`e` or `E`) with an integer (e.g. `2e10`, `-90E3`, `53.5e93`).

Invalid cases include extra characters, misplaced signs, multiple exponents/dots, or missing digits.

The approach:

- Split the string on `'e'` or `'E'` (at most one allowed).
- Validate the base (must be integer or decimal).
- Validate the exponent (if present, must be an integer).
- Carefully handle optional sign characters and digit/dot placement.

---

### 🪜 Steps

1. **Trim and preprocess**: Remove leading/trailing spaces.
2. **Check exponent**: Split into `base` and `exponent` parts by `'e'`/`'E'`. If more than one split → invalid.
3. **Validate base**:
   - Must be integer or decimal (handle `+`, `-`, and `.` rules).
   - Ensure at least one digit is present.
4. **Validate exponent** (if present):
   - Must be an integer with optional sign.
   - No decimal points allowed.
5. **Return result**: True if both base and exponent (if any) are valid.

---

## ✅ Constraints

- String length: `1 <= s.length <= 200`
- Characters allowed: digits (`0-9`), sign (`+`, `-`), decimal point (`.`), exponent (`e`/`E`), and spaces (only leading/trailing).
- Must contain **at least one digit** overall.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)** — we scan the string once and validate components |
| Space Complexity  | **O(1)** — constant extra space |

---
