# 🧩 LeetCode Problem: Merge k Sorted Lists

- **Problem Link**: [Merge k Sorted Lists – LeetCode](https://leetcode.com/problems/merge-k-sorted-lists/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/merge-k-sorted-lists/solutions/)

---

## 🧠 Algorithm Explanation

The problem requires merging `k` sorted linked lists into a single sorted linked list.

To solve this efficiently, we use a **min-heap (priority queue)** to always extract the smallest current head among the `k` lists. At each step, we attach the smallest node to our result list and push the next node (if available) from the same list into the heap. This guarantees that the merged list is built in sorted order with optimal efficiency.

We store tuples of the form `(node value, list index, node)` in the heap to prevent errors when two node values are equal (since `ListNode` objects are not directly comparable).

---

### 🪜 Steps

1. **Initialize** an empty min-heap.
2. **Push** the head node of each non-empty list into the heap as a tuple `(val, index, node)`.
3. **Pop** the smallest node from the heap and add it to the merged list.
4. **Push** the next node from the same list (if it exists) into the heap.
5. **Repeat** steps 3–4 until the heap is empty.
6. **Return** the merged linked list starting from `dummy.next`.

---

## ✅ Constraints

- `k == lists.length`
- `0 <= k <= 10⁴`
- `0 <= lists[i].length <= 500`
- `-10⁴ <= lists[i][j] <= 10⁴`
- Each `lists[i]` is sorted in ascending order.
- The total number of nodes across all lists is ≤ 10⁵.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity     |
|-------------------|----------------|
| Time Complexity   | O(N log k)     |
| Space Complexity  | O(k)           |

- `N` is the total number of nodes in all linked lists.
- `log k` comes from maintaining a heap of size `k`.

---
