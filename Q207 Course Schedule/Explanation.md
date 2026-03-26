# 🧩 LeetCode Problem: Course Schedule

- **Problem Link**: [Course Schedule – LeetCode](https://leetcode.com/problems/course-schedule/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/course-schedule/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks whether it is possible to finish all courses given prerequisite relationships.  
This can be modeled as a **directed graph**:

- Each course is a node.
- A prerequisite pair `[a, b]` means there is a directed edge `b -> a`.
- If the graph contains a **cycle**, then it is impossible to complete all courses.
- If the graph has **no cycle**, then all courses can be completed.

The optimal approach is **Topological Sorting using Kahn’s Algorithm**.

Why this algorithm is used:

- It efficiently detects whether a directed graph contains a cycle.
- It processes all courses with no prerequisites first.
- If we can process all nodes, there is no cycle.
- If some nodes are left unprocessed, a cycle exists.

---

### 🪜 Steps

1. **Build the graph and indegree array**:  
   For each prerequisite `[a, b]`, add an edge `b -> a` and increment `indegree[a]`.

2. **Start with all courses having indegree 0**:  
   These are courses that can be taken immediately since they have no remaining prerequisites.

3. **Process the queue**:  
   Repeatedly remove a course from the queue, count it as completed, and reduce the indegree of its neighbors.  
   If any neighbor’s indegree becomes 0, add it to the queue.

4. **Check if all courses were processed**:  
   If the number of processed courses equals `numCourses`, return `true`; otherwise return `false`.

---

## ✅ Constraints

- `1 <= numCourses <= 2000`
- `0 <= prerequisites.length <= 5000`
- `prerequisites[i].length == 2`
- `0 <= a_i, b_i < numCourses`
- All the pairs `prerequisites[i]` are unique.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(V + E)   |
| Space Complexity  | O(V + E)   |

Where:

- `V = numCourses`
- `E = prerequisites.length`

---
