from collections import defaultdict, deque


class Solution:
    def findLadders(self, beginWord: str, endWord: str, wordList: list[str]) -> list[list[str]]:
        word_set = set(wordList)
        if endWord not in word_set:
            return []

        # Distance from beginWord to each word
        dist = {beginWord: 0}
        # Parents map: word -> list of previous words on shortest paths
        parents = defaultdict(list)

        q = deque([beginWord])
        found_end = False
        word_len = len(beginWord)

        while q and not found_end:
            # To avoid revisiting words in the same level multiple times
            current_level_visited = {}
            for _ in range(len(q)):
                word = q.popleft()
                current_dist = dist[word]

                # Try all one-letter transformations
                for i in range(word_len):
                    for c in "abcdefghijklmnopqrstuvwxyz":
                        if c == word[i]:
                            continue
                        next_word = word[:i] + c + word[i+1:]
                        if next_word not in word_set:
                            continue

                        # If this next_word hasn't been seen before in dist
                        if next_word not in dist:
                            dist[next_word] = current_dist + 1
                            parents[next_word].append(word)
                            q.append(next_word)
                            current_level_visited[next_word] = True
                        # If we found another shortest way to reach next_word
                        elif dist[next_word] == current_dist + 1:
                            parents[next_word].append(word)

                        if next_word == endWord:
                            found_end = True

            # Optional optimization: remove words visited in this level from word_set
            # so they won't be revisited in later levels.
            for w in current_level_visited:
                if w in word_set:
                    word_set.remove(w)

        if endWord not in dist:
            return []

        # Backtrack from endWord to beginWord using parents map
        res = []
        path = [endWord]

        def dfs(word: str):
            if word == beginWord:
                res.append(path[::-1])  # reverse to get begin -> end
                return
            for p in parents[word]:
                path.append(p)
                dfs(p)
                path.pop()

        dfs(endWord)
        return res
    