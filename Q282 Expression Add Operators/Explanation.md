# LeetCode Problem: Expression Add Operators

- **Problem Link**: [Expression Add Operators – LeetCode](https://leetcode.com/problems/expression-add-operators/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/expression-add-operators/solutions/)

---

## Algorithm

Insert `+`, `-`, or `*` between digits of `num` (or group consecutive digits into
multi-digit operands) so the expression evaluates to `target`. Operands must
not have leading zeros.

With `num.length <= 10`, enumerate all expressions by **backtracking**:

1. At each position, pick the next operand (one or more digits; stop extending
   if you would create a leading zero).
2. If this is the first operand, start the path and set both running value and
   **last operand** to that number.
3. Otherwise append `+`, `-`, or `*`:
   - `+` / `-`: update running value normally; last operand is `+value` or
     `-value`.
   - `*`: undo the effect of the last operand on the running value, then apply
     multiplication:  
     `new_val = curr_val - last_operand + last_operand * value`  
     `new_last = last_operand * value`

4. When the whole string is consumed, if `curr_val == target`, add `path` to the
   answer.

This avoids re-parsing the expression at each leaf and handles `*` precedence
correctly.

### Implementation notes

- **Incremental parsing**: build each operand digit-by-digit instead of
  reslicing the string.
- **In-place path building**: append to a buffer and undo with slice truncation
  after each branch, instead of copying the full expression string every step.
- **No operator pruning**: skipping `+` or `-` based only on the running sum
  can drop valid answers because `*` can change precedence later.

---

## Constraints

- `1 <= num.length <= 10`
- `num` contains only digits
- `-2^31 <= target <= 2^31 - 1`
- No operand may have leading zeros

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(4^n)** — branch over operators and operand lengths; `n <= 10` is small      |
| Space Complexity   | **O(n)** — recursion depth and current path string                              |

---
<!-- markdownlint-enable MD013 -->
