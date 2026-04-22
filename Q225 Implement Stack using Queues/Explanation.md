# LeetCode Problem: Implement Stack using Queues

- **Problem Link**: [Implement Stack using Queues – LeetCode](https://leetcode.com/problems/implement-stack-using-queues/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/implement-stack-using-queues/solutions/)

---

## Algorithm

Use **one queue** and keep its **front** aligned with the **stack top** (last
pushed value).

**`push(x)`:** append `x` at the back, then **rotate** the queue \(n - 1\)
times: repeatedly move the front element to the back. After rotation, `x` sits
at the front while older values stay behind it in FIFO order—exactly LIFO for
the logical stack.

**`pop` / `top`:** read or remove the **front** of the queue in **O(1)** time.

**`empty`:** queue is empty.

Only standard queue behavior is needed: enqueue at back, dequeue from front,
size / empty checks.

---

## Constraints

- `1 <= x <= 9`
- At most `100` calls to `push`, `pop`, `top`, and `empty`
- Every `pop` and `top` runs on a **non-empty** stack

---

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n)** per `push` (rotate queue); **O(1)** for `pop`, `top`, `empty`          |
| Space Complexity   | **O(n)** — one queue stores every stack element                                 |

---
