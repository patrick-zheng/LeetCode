from collections import defaultdict, deque


class Solution:
    def ladderLength(self, beginWord: str, endWord: str, wordList: list[str]) -> int:
        if endWord not in wordList:
            return 0

        L = len(beginWord)

        pattern_map = defaultdict(list)
        for word in wordList:
            for i in range(L):
                pattern = word[:i] + "*" + word[i+1:]
                pattern_map[pattern].append(word)

        queue = deque([(beginWord, 1)])
        visited = set([beginWord])

        while queue:
            current_word, depth = queue.popleft()

            for i in range(L):
                pattern = current_word[:i] + "*" + current_word[i+1:]
                for neighbor in pattern_map[pattern]:
                    if neighbor == endWord:
                        return depth + 1
                    if neighbor not in visited:
                        visited.add(neighbor)
                        queue.append((neighbor, depth + 1))

                pattern_map[pattern] = []

        return 0
    