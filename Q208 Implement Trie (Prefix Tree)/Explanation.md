# 🧩 LeetCode Problem: Implement Trie (Prefix Tree)

- **Problem Link**: [Implement Trie (Prefix Tree) – LeetCode](https://leetcode.com/problems/implement-trie-prefix-tree/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/implement-trie-prefix-tree/solutions/)

---

## 🧠 Algorithm Explanation

A **Trie** is a tree-like data structure used to store strings efficiently, especially when many strings share common prefixes.

Each node represents a character, and paths from the root to a node represent prefixes or complete words.

This algorithm is used because it allows:

- **Insertion** of a word in linear time relative to the word length.
- **Exact search** of a word in linear time.
- **Prefix search** in linear time.

Instead of comparing against every stored word, we move character by character through the tree, which makes trie operations very efficient for dictionary-like problems.

---

### 🪜 Steps

1. **Create a trie node structure**  
   Each node stores:
   - children references for next characters
   - a boolean to mark whether a word ends at that node

2. **Insert a word**  
   Start from the root and process each character:
   - if the character path does not exist, create it
   - move to the next node
   - after the last character, mark the node as the end of a word

3. **Search / Prefix check**  
   - For `search(word)`, follow the path of each character and return `true` only if the full path exists and the last node is marked as a complete word
   - For `startsWith(prefix)`, follow the path of each character and return `true` if the full prefix path exists

---

## ✅ Constraints

- `1 <= word.length, prefix.length <= 2000`
- `word` and `prefix` consist only of lowercase English letters
- At most `3 * 10^4` calls in total will be made to `insert`, `search`, and `startsWith`

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                   |
|-------------------|------------------------------|
| Time Complexity   | `O(L)` per operation         |
| Space Complexity  | `O(N * L)` in the worst case |

Where:

- `L` = length of the word or prefix
- `N` = number of inserted words

---
