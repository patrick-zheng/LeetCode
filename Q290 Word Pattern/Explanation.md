# LeetCode Problem: Word Pattern

- **Problem Link**: [Word Pattern – LeetCode](https://leetcode.com/problems/word-pattern/)
- **Solution Link**: [Official Solutions](https://leetcode.com/problems/word-pattern/solutions/)

---

## Algorithm

Return **true** if `pattern` matches string `s` when each distinct character
in `pattern` maps to a distinct word in `s`, and the mapping is consistent
for every position.

Example: `pattern = "abba"`, `s = "dog cat cat dog"` → **true** (`a`→`dog`,
`b`→`cat`).

1. Split `s` into words (single-space separated).
2. If `len(pattern) != len(words)`, return **false**.
3. Maintain two hash maps:
   - `char_to_word`: pattern character → word
   - `word_to_char`: word → pattern character
4. For each pair `(ch, word)`:
   - If `ch` is already mapped, its word must equal `word`.
   - Else if `word` is already mapped to another character, return **false**.
   - Else record both mappings.

This enforces a **bijection** between pattern letters and words in one pass.

---

## Constraints

- `1 <= pattern.length <= 300`
- `pattern` contains only lowercase English letters
- `1 <= s.length <= 3000`
- `s` contains only lowercase English letters and spaces
- `s` has no leading or trailing spaces
- Words in `s` are separated by a single space

---

<!-- markdownlint-disable MD013 -->
## Complexity

| Metric             | Complexity                                                                      |
|--------------------|---------------------------------------------------------------------------------|
| Time Complexity    | **O(n + m)** — split `s` and scan `pattern` once; `n` words, `m` pattern length |
| Space Complexity   | **O(n)** — hash maps for at most `n` distinct letter/word pairs                 |

---
<!-- markdownlint-enable MD013 -->
