# 🧩 LeetCode Problem: Partition List

- **Problem Link**: [Partition List – LeetCode](https://leetcode.com/problems/partition-list/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/partition-list/solutions/)

---

## 🧠 Algorithm Explanation

We need to partition the linked list into two parts:  

- Nodes with values **less than x**  
- Nodes with values **greater than or equal to x**

while **preserving the original relative order** within each partition.

The optimal way is to use two dummy lists:  

- One for nodes `< x` (the "before" list)  
- One for nodes `>= x` (the "after" list)  

We iterate through the original list, appending each node to the correct list. Finally, we stitch the two lists together.

This avoids extra allocations and ensures O(1) space overhead.

---

### 🪜 Steps

1. **Initialize two dummy nodes**: one for the "before" list and one for the "after" list.  
2. **Traverse the linked list**:  
   - If `node.val < x`, append it to the "before" list.  
   - Otherwise, append it to the "after" list.  
3. **Connect the partitions**: attach the tail of the "before" list to the head of the "after" list.  
4. **Return result**: the head of the "before" list.  

---

## ✅ Constraints

- The number of nodes in the list is in the range `[0, 200]`.  
- `-100 <= Node.val <= 100`  
- `-200 <= x <= 200`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(1)       |

---
