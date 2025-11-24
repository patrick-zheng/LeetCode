# 🧩 LeetCode Problem: Word Ladder II

- **Problem Link**: [Word Ladder II – LeetCode](https://leetcode.com/problems/word-ladder-ii/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-ladder-ii/solutions/)

---

## 🧠 Algorithm Explanation

We treat each word as a node in an unweighted graph, where there is an edge between two words if they differ by exactly **one letter**.

We want **all shortest paths** from `beginWord` to `endWord`. For unweighted graphs, shortest paths are naturally found using **Breadth-First Search (BFS)**. However, instead of just tracking distances, we also need to recover **all shortest transformation sequences**.

We do this in two phases:

1. **BFS phase (forward)**  
   - Compute the shortest distance from `beginWord` to every reachable word.
   - Maintain a `parents` map where `parents[w]` stores all words that can reach `w` along some shortest path.

2. **DFS / Backtracking phase (reverse)**  
   - Starting from `endWord`, recursively move backwards through the `parents` map until we reach `beginWord`.
   - Each successful backtrack path is reversed to form a valid sequence from `beginWord` to `endWord`.

This combination ensures we:

- Use BFS to guarantee minimal path length.
- Use backtracking to enumerate **all** sequences of that minimal length.

---

### 🪜 Steps

1. **Setup & Validation**
   - Insert all words from `wordList` into a `set` (`word_set`) for O(1) lookups.
   - If `endWord` is **not in** `word_set`, return `[]` immediately (no possible transformation).

2. **BFS to Find Distances & Parents**
   - Initialize:
     - `dist[beginWord] = 0`
     - Queue `q` with `beginWord`.
     - `parents` as `defaultdict(list)`.
   - While the queue is not empty:
     - For each word in the current level:
       - For each position `i` in the word:
         - Try replacing the character at `i` with `'a'..'z'` to form `next_word`.
         - If `next_word` is in `word_set`:
           - If `next_word` not in `dist`:
             - Set `dist[next_word] = dist[word] + 1`.
             - Append `word` to `parents[next_word]`.
             - Push `next_word` into the queue.
           - Else if `dist[next_word] == dist[word] + 1`:
             - Append `word` to `parents[next_word]` (another shortest parent).
     - Optionally remove all words visited at this level from `word_set` to avoid revisiting them later.

3. **Check Reachability**
   - If `endWord` is not in `dist` after BFS, return `[]` (no sequence exists).

4. **DFS / Backtracking to Build Paths**
   - Start with `path = [endWord]`.
   - Define a recursive `dfs(word)`:
     - If `word == beginWord`, append `path[::-1]` to the result list (since we built it backwards).
     - Otherwise, for each `parent` in `parents[word]`:
       - Append `parent` to `path`, call `dfs(parent)`, and then pop it.
   - Call `dfs(endWord)` and return the collected list of paths.

---

## ✅ Constraints

Typical constraints from the problem statement:

- `1 <= beginWord.length <= 10`
- `endWord.length == beginWord.length`
- `1 <= wordList.length <= 5000`
- `beginWord`, `endWord`, and all `wordList` words consist of **lowercase English letters**.
- `beginWord != endWord`
- All words in `wordList` are **unique**.

---

## ⏱ Time and Space Complexity

Let:

- `N` = number of words in `wordList`
- `L` = length of each word

| Metric            | Complexity              |
|-------------------|-------------------------|
| Time Complexity   | O(N · L · 26) ≈ O(NL)  |
| Space Complexity  | O(N · L)               |

- Each word can generate up to `26 * L` neighboring words (one-letter transformations).
- BFS and the `parents` structure together store information for at most `N` words.
- The DFS/backtracking explores all shortest sequences (which can be many in degenerate cases).

---
