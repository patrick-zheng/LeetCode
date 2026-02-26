# 🧩 LeetCode Problem: Repeated DNA Sequences

- **Problem Link**: [Repeated DNA Sequences – LeetCode](https://leetcode.com/problems/repeated-dna-sequences/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/repeated-dna-sequences/solutions/)

---

## 🧠 Algorithm Explanation

Since the DNA string contains only four characters (`A, C, G, T`), we can encode each character using **2 bits**:

- A → 00  
- C → 01  
- G → 10  
- T → 11  

A substring of length 10 therefore requires **20 bits**, which fits inside a 32-bit integer.  

We use a **rolling hash with bit manipulation**:

- Slide a window of length 10 across the string.
- Maintain a rolling 20-bit integer representation.
- Store previously seen hashes in a set.
- If we encounter the same hash again, we add the substring to the result.

This avoids repeated substring construction and keeps the solution efficient.

Why this approach?

- Brute force substring comparison would be expensive.
- Hashing with bit encoding guarantees **O(n)** time.
- Small fixed alphabet (4 letters) makes bit compression ideal.

---

### 🪜 Steps

1. **Initialize Encoding**
   - Map each DNA character to a 2-bit value.
   - Prepare a bitmask to keep only the last 20 bits.

2. **Sliding Window**
   - Traverse the string.
   - Update rolling hash by shifting left 2 bits and adding new character.
   - Apply mask to discard old bits.

3. **Detect Repeats**
   - If hash already exists in `seen`, add substring to result (only once).
   - Otherwise, insert into `seen`.

---

## ✅ Constraints

- 1 ≤ s.length ≤ 10⁵
- `s[i]` ∈ { 'A', 'C', 'G', 'T' }
- Return all 10-letter-long sequences that occur more than once.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(n)       |
| Space Complexity  | O(n)       |

- Each character processed once.
- Each window encoded in constant time.
- Hash sets store at most O(n) entries.
