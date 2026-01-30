# 🧩 LeetCode Problem: Fraction to Recurring Decimal

- **Problem Link**: [Fraction to Recurring Decimal – LeetCode](https://leetcode.com/problems/fraction-to-recurring-decimal/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/fraction-to-recurring-decimal/solutions/)

---

## 🧠 Algorithm Explanation

This problem is solved using **long division simulation** with **cycle detection**.

A decimal becomes repeating **iff the remainder repeats** during division.  
So the idea is to simulate the division process digit-by-digit and store each remainder’s position in the result string.  
When the same remainder appears again, the digits between the two occurrences form the repeating cycle — which must be wrapped in parentheses.

This method is optimal because:

- It directly mirrors real division
- Guarantees correct detection of repeating patterns
- Avoids floating point precision errors
- Works for very large numbers safely using integer arithmetic

---

### 🪜 Steps

1. **Handle sign and integer part**
   - Determine sign (`-` if numerator and denominator have opposite signs)
   - Compute integer part using integer division

2. **Check for finite decimal**
   - If remainder = 0 → return result immediately (no decimal part)

3. **Simulate long division with remainder tracking**
   - Multiply remainder by 10
   - Extract digit (`remainder / denominator`)
   - Track each remainder in a map:  
     `remainder → index in output string`

4. **Detect repetition**
   - If a remainder repeats:
     - Insert `"("` at the first occurrence index
     - Append `")"` at the end
     - Stop

---

## ✅ Constraints

- Numerator and denominator are integers
- Denominator ≠ 0
- Output length < `10^4`
- Must return finite decimal if possible
- Must detect and format repeating decimals
- Multiple correct formats allowed for repeating decimals

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(L)**   |
| Space Complexity  | **O(L)**   |

Where **L** is the number of digits in the output string (≤ 10⁴).

---
