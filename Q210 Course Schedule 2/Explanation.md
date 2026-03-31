# 🧩 LeetCode Problem: Course Schedule II

- **Problem Link**: [Course Schedule II – LeetCode](https://leetcode.com/problems/course-schedule-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/course-schedule-ii/solutions/)

---

## 🧠 Algorithm Explanation

This problem asks for a valid order to take all courses while respecting prerequisite relationships.

This is a **topological sorting** problem on a **directed graph**:

- Each course is a node
- A prerequisite pair `[a, b]` means there is a directed edge `b -> a`
- We need an ordering where every course appears only after all its prerequisites

The optimal approach is **Kahn’s Algorithm (BFS topological sort)**.

Why this algorithm is used:

- It naturally builds a valid course order
- It detects cycles automatically
- If a cycle exists, it is impossible to finish all courses, so we return an empty array
- It runs efficiently in **O(V + E)** time, which is ideal for the constraints

---

### 🪜 Steps

1. **Build the graph and indegree array**  
   Create an adjacency list where `b -> a` for each prerequisite pair.  
   Also keep an `indegree` array where `indegree[a]` counts how many prerequisites course `a` still has.

2. **Start with all courses having no prerequisites**  
   Push every course with `indegree = 0` into a queue.  
   These are the courses we can take immediately.

3. **Process the queue**  
   Repeatedly:
   - pop a course from the queue
   - add it to the answer
   - reduce the indegree of its neighbors
   - if any neighbor’s indegree becomes `0`, push it into the queue

4. **Check if all courses were processed**  
   - If the result contains all `numCourses` courses, return it
   - Otherwise, a cycle exists, so return `[]`

---

## ✅ Constraints

- `1 <= numCourses <= 2000`
- `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
- `prerequisites[i].length == 2`
- `0 <= ai, bi < numCourses`
- `ai != bi`
- All pairs `[ai, bi]` are distinct

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(V + E)   |
| Space Complexity  | O(V + E)   |

### Where:

- `V = numCourses`
- `E = prerequisites.length`

The graph is built once, and each node and edge is processed at most once.

---
