# LeetCode Problem: Implement Queue using Stacks

- **Problem Link**: [Implement Queue using Stacks – LeetCode](https://leetcode.com/problems/implement-queue-using-stacks/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/implement-queue-using-stacks/solutions/)

---

## Algorithm

Use **two stacks**: one for **enqueue** (new arrivals) and one for **dequeue**
(the front of the queue appears at the **top** of the second stack).

**`push(x)`:** push onto the **in** stack only — **O(1)**.

**`pop` / `peek`:** if the **out** stack is empty, **drain** the **in** stack
into **out** by repeatedly popping from **in** and pushing onto **out**. The
order reverses twice relative to arrival order, so the **oldest** pushed value
ends up on **top** of **out**. Then `pop` removes that top; `peek` reads it
without removing. If **out** was already non-empty, skip draining — those
elements are still ahead in FIFO order.

**`empty`:** both stacks are empty.

Each element moves from **in** to **out** at most **once** over the whole
sequence, so **n** operations cost **O(n)** total — **amortized O(1)** per
operation even though a single `pop` or `peek` can cost **O(n)** when **out**
was empty.

---

## Constraints

- `1 <= x <= 9`
- At most `100` calls will be made to `push`, `pop`, `peek`, and `empty`
- All calls to `pop` and `peek` are valid

---

<!-- markdownlint-disable MD013 -->

## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(1)** amortized; worst **O(n)** when emptying the input stack to output      |
| Space Complexity   | **O(n)** — elements split across two stacks until dequeued from the front       |

<!-- markdownlint-enable MD013 -->

---
