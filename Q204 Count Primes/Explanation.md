# 🧩 LeetCode Problem: Count Primes

- **Problem Link**: [Count Primes – LeetCode](https://leetcode.com/problems/count-primes/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/count-primes/solutions/)

---

## 🧠 Algorithm Explanation

The optimal solution uses the **Sieve of Eratosthenes**.

Instead of checking each number one by one for primality, we create an array that keeps track of whether each number is prime. Then, starting from `2`, we mark all multiples of each prime number as non-prime. By the end, the numbers still marked as prime are exactly the prime numbers less than `n`.

This algorithm is used because it is much faster than testing every number individually, especially since `n` can be as large as `5 * 10^6`.

---

### 🪜 Steps

1. **Initialize a boolean array**  
   Create an array `is_prime` of size `n`, and assume every number is prime at first. Then mark `0` and `1` as non-prime.

2. **Mark multiples as non-prime**  
   For each number `p` starting from `2`, if it is still marked as prime, mark all multiples of `p` starting from `p * p` as non-prime.

3. **Count remaining primes**  
   After processing all numbers up to `sqrt(n)`, count how many entries are still marked as prime. That count is the answer.

---

## ✅ Constraints

- `0 <= n <= 5 * 10^6`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity      |
|-------------------|-----------------|
| Time Complexity   | O(n log log n)  |
| Space Complexity  | O(n)            |

---
