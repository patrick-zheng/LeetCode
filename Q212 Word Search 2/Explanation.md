# 🧩 LeetCode Problem: Word Search II

- **Problem Link**: [Word Search II – LeetCode](https://leetcode.com/problems/word-search-ii/)
- **Solution Link**: [Word Search II – Solutions](https://leetcode.com/problems/word-search-ii/solutions/)

---

## 🧠 Algorithm Explanation

Find every word from the dictionary that appears on the board as a path of adjacent (up, down, left, right) cells, without reusing a cell in the same path.

Running “Word Search I” separately for each word repeats the same prefix work many times. **Use a trie (prefix tree):** insert all `words`, then DFS from each cell while walking the trie. If the next board letter is not a child of the current trie node, stop—**early termination** for invalid prefixes.

Mark visited cells in place (e.g. replace with `'#'`), then restore on backtracking—**O(1)** extra grid space.

**Pruning:** After a word is taken from a node and it has no remaining children, remove that node’s link from its parent so later searches skip dead branches.

---

### 🪜 Steps

1. **Build the trie**: Insert every word; store the full string at terminal nodes so matches are recorded when DFS reaches that node.

2. **Scan the board**: For each cell `(i, j)`, start DFS from the trie root as the parent (state before consuming `board[i][j]`).

3. **DFS body**: Read `c = board[i][j]`. If `c` is the visit marker or the parent has no trie child for `c`, return. Descend to that child. If it holds a word, push it to the answer and clear it so duplicates are not reported again.

4. **Recurse**: Set `board[i][j]` to the visit marker; DFS all four neighbors from the **child** trie node.

5. **Backtrack**: Restore `board[i][j]`. If the child has no children and no word left, remove it from the parent (prune).

6. **Return** the list of found words.

---

## ✅ Constraints

- `m == board.length`, `n == board[i].length`
- `1 <= m, n <= 12`
- `1 <= words.length <= 3 * 10^4`
- `1 <= words[i].length <= 10`
- `words[i]` is lowercase English letters; all `words` are **unique**
- `board[i][j]` is a lowercase English letter

---

## ⏱ Time and Space Complexity

Let `S` = total characters in all words, `L` = max word length, `N = m * n`. Worst-case grid DFS is `O(N · 4^L)`; trie build is `O(S)`. Pruning and prefix checks usually do much better in practice.

| Metric            | Complexity |
|-------------------|------------|
| Time Complexity   | O(N · 4^L + S) |
| Space Complexity  | O(S) trie + O(L) recursion stack |

---
