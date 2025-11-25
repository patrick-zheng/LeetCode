# 🧩 LeetCode Problem: Word Ladder

- **Problem Link**: [Word Ladder – LeetCode](https://leetcode.com/problems/word-ladder/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-ladder/solutions/)

---

## 🧠 Algorithm Explanation

We model the problem as a **shortest path in an unweighted graph**:

- Each **word** (including `beginWord` and all words in `wordList`) is a **node**.
- There is an **edge** between two words if they differ by **exactly one character**.
- We want the **length of the shortest transformation sequence** from `beginWord` to `endWord`.

Because every edge has the same cost (1 step), we use **Breadth-First Search (BFS)** to find the shortest path. BFS guarantees that the first time we reach `endWord`, we’ve used the minimum number of transformations.

To avoid O(N²) comparisons between words, we use an **intermediate pattern map**:

- For a word like `"hot"`, we generate patterns:
  - `*ot`, `h*t`, `ho*`
- We build a map:  
  `pattern -> [list of words with that pattern]`

During BFS, from a word we:

1. Generate all its patterns.
2. For each pattern, retrieve all words that match it from the map → these are its neighbors.
3. Visit each unvisited neighbor and push it into the queue with depth + 1.

The BFS depth when we first see `endWord` is the answer.

---

### 🪜 Steps

1. **Check reachability**
   - If `endWord` is **not in** `wordList`, return `0` immediately (no valid sequence).

2. **Preprocess patterns**
   - Let `L` be the word length.
   - For each word in `wordList`:
     - For each index `i` from `0` to `L - 1`:
       - Replace the character at `i` with `'*'` to form a pattern.
       - Add the word to `pattern_map[pattern]`.

3. **Run BFS**
   - Initialize:
     - Queue: `[(beginWord, 1)]` where `1` is the initial sequence length.
     - Visited set: `{beginWord}`.
   - While the queue is not empty:
     1. Pop `(current_word, depth)`.
     2. For each position `i` in `[0, L-1]`, build the pattern with `'*'` at position `i`.
     3. For each `neighbor` in `pattern_map[pattern]`:
        - If `neighbor == endWord`, return `depth + 1`.
        - If `neighbor` is not visited:
          - Mark it visited.
          - Push `(neighbor, depth + 1)` into the queue.
     4. After processing a pattern, clear `pattern_map[pattern]` to avoid repeated work.

4. **No sequence**
   - If BFS ends without reaching `endWord`, return `0`.

---

## ✅ Constraints

- `1 <= wordList.length <= 5000`
- `1 <= beginWord.length <= 10`
- `endWord.length == beginWord.length`
- `beginWord`, `endWord`, and all `wordList[i]` consist of **lowercase English letters**.
- All words in `wordList` are **unique**.

---

## ⏱ Time and Space Complexity

Let:

- `N` = number of words in `wordList`
- `L` = length of each word

| Metric            | Complexity  |
|-------------------|-------------|
| Time Complexity   | `O(N * L)`  |
| Space Complexity  | `O(N * L)`  |

**Reasoning:**

- Preprocessing: for each of `N` words we generate `L` patterns → `O(N * L)`.
- BFS: each word is visited at most once; for each word we generate `L` patterns and process each pattern list once overall (we clear lists after use) → `O(N * L)`.

---
