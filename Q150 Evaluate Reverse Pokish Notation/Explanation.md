# 🧩 LeetCode Problem: Evaluate Reverse Polish Notation

- **Problem Link**: <https://leetcode.com/problems/evaluate-reverse-polish-notation/>
- **Solution Link**: <https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/>

---

## 🧠 Algorithm Explanation

This problem is best solved using a **stack-based approach**, which naturally fits the evaluation of expressions written in **Reverse Polish Notation (RPN)**.

In RPN, operators always come **after** their operands. This allows us to process the expression from left to right and evaluate sub-expressions as soon as an operator is encountered. A stack is ideal for temporarily storing operands until an operator is applied.

The algorithm ensures:

- Correct operator precedence without extra logic
- Efficient evaluation in a single pass
- Proper handling of integer division that truncates toward zero

---

### 🪜 Steps

1. **Initialize a stack**
   - This stack will store integer operands.

2. **Iterate through each token**
   - If the token is a number, convert it to an integer and push it onto the stack.
   - If the token is an operator (`+`, `-`, `*`, `/`):
     - Pop the top two elements from the stack (second pop is the left operand).
     - Apply the operator.
     - Push the result back onto the stack.

3. **Return the final result**
   - After processing all tokens, the stack will contain exactly one value.
   - This value is the result of the expression.

---

## ✅ Constraints

- `1 <= tokens.length <= 10⁴`
- Each token is either an operator or a valid integer
- Division by zero will never occur
- All intermediate results fit within a 32-bit signed integer
- Division between two integers truncates toward zero

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n)**   |
| Space Complexity  | **O(n)**   |

- **Time Complexity**: Each token is processed exactly once.
- **Space Complexity**: In the worst case, all operands are stored on the stack.

---
