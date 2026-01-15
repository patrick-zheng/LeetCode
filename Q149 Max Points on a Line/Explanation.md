# 🧩 LeetCode Problem: Max Points on a Line

- **Problem Link**: <https://leetcode.com/problems/max-points-on-a-line/>
- **Solution Link**: <https://leetcode.com/problems/max-points-on-a-line/solutions/>

---

## 🧠 Algorithm Explanation

The goal is to find the **maximum number of points that lie on the same straight line**.  
The most optimal and reliable approach is to fix one point as an **anchor** and compute the slope it forms with every other point.

If multiple points have the **same slope** relative to the anchor, they lie on the same line.

Floating-point slopes are avoided due to precision issues. Instead, slopes are represented as **normalized integer pairs** `(dy, dx)` reduced by their **greatest common divisor (GCD)**.

This approach is optimal because any solution must examine point-to-point relationships, leading to a tight **Θ(n²)** lower bound.

---

### 🪜 Steps

1. **Fix an anchor point**
   - Iterate through each point and treat it as the starting point of a line.

2. **Count slopes**
   - For the anchor point, compute slopes to all other points.
   - Normalize slopes using `(dy / gcd, dx / gcd)`.
   - Handle special cases:
     - Vertical lines `(1, 0)`
     - Horizontal lines `(0, 1)`
     - Duplicate points (same coordinates)

3. **Track the best line**
   - Use a hash map to count occurrences of each slope.
   - The maximum count for one slope + duplicates + anchor point gives the best line through that anchor.
   - Keep a global maximum.

---

## ✅ Constraints

- `1 ≤ points.length ≤ 300`
- `points[i].length == 2`
- `-10⁴ ≤ xi, yi ≤ 10⁴`
- All points are integers

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | **O(n²)**  |
| Space Complexity  | **O(n)**   |

---

## 🧪 Why This Is Optimal

- Any algorithm must inspect pairwise relationships between points.
- Hashing normalized slopes avoids floating-point errors.
- Early termination and reduced comparisons (`j > i`) optimize performance in practice.
- This is the **best possible asymptotic solution** for arbitrary points.

---
