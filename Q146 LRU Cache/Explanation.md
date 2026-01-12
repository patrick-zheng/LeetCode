# 🧩 LeetCode Problem: LRU Cache

- **Problem Link**: [LRU Cache – LeetCode](https://leetcode.com/problems/lru-cache/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/lru-cache/solutions/)

---

## 🧠 Algorithm Explanation

This problem requires designing a cache that supports **fast access** and **fast updates** while maintaining the **Least Recently Used (LRU)** eviction policy.

To achieve **O(1)** average time complexity for both `get` and `put`, we use a combination of:

- **Hash Map** for constant-time key lookups
- **Ordered data structure** to track usage order

In Python, `OrderedDict` is ideal because it maintains insertion order and allows moving elements to the end in O(1), which directly maps to the LRU behavior.

---

### 🪜 Steps

1. **Initialization**
   - Store the cache capacity.
   - Use an `OrderedDict` to store key-value pairs in usage order.

2. **Get Operation**
   - If the key does not exist, return `-1`.
   - If it exists, move the key to the end (mark as most recently used) and return its value.

3. **Put Operation**
   - If the key already exists, update its value and mark it as most recently used.
   - If it is a new key, insert it.
   - If capacity is exceeded, remove the **least recently used** item (the first element).

---

## ✅ Constraints

- `1 ≤ capacity ≤ 3000`
- `0 ≤ key ≤ 10⁴`
- `0 ≤ value ≤ 10⁵`
- At most `2 × 10⁵` calls to `get` and `put`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(1)       |
| Space Complexity  | O(capacity)|

---
