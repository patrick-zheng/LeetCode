# 🧩 LeetCode Problem: Design Add and Search Words Data Structure

- **Problem Link**: [Design Add and Search Words Data Structure – LeetCode](https://leetcode.com/problems/design-add-and-search-words-data-structure/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/)

---

## 🧠 Algorithm Explanation

The optimal solution uses a **Trie (prefix tree)** combined with **DFS** for wildcard search.

A Trie is used because:

- It stores words efficiently character by character.
- Adding a word is fast since we only walk through its letters once.
- Searching normal words is fast for the same reason.
- For the `.` wildcard, DFS allows us to try all possible child letters at that position.

When searching:

- If the current character is a normal letter, move to that specific child.
- If the current character is `.`, recursively try every possible child.
- If we reach the end of the word, we only return `true` if the current node marks the end of a stored word.

This is optimal because it avoids comparing the search word against every inserted word.

---

### 🪜 Steps

1. **Insert words into a Trie**  
   For each character in the word, create the child node if it does not already exist, then move forward. Mark the last node as an end-of-word node.

2. **Search normally for letters**  
   If the current character is a lowercase letter, move only to that matching child node. If it does not exist, return `false`.

3. **Use DFS for `.` wildcard**  
   If the current character is `.`, recursively try every child node. If any path reaches a valid end-of-word match, return `true`.

---

## ✅ Constraints

- `1 <= word.length <= 25`
- `word` in `addWord` consists of lowercase English letters.
- `word` in `search` consists of `'.'` or lowercase English letters.
- There will be at most `2` dots in a `search` query.
- At most `10^4` calls will be made to `addWord` and `search`.

---

## ⏱ Time and Space Complexity

| Metric            | Complexity                                           |
|-------------------|------------------------------------------------------|
| Time Complexity   | `addWord`: O(L), `search`: O(26^k * L) in worst case |
| Space Complexity  | O(N × L)                                             |

Where:

- `L` = length of the word
- `N` = number of inserted words
- `k` = number of `.` wildcards

Since the problem guarantees at most 2 dots, search remains efficient in practice.

---
