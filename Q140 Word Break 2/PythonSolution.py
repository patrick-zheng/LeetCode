from functools import lru_cache


class Solution:
    def wordBreak(self, s: str, wordDict: list[str]) -> list[str]:
        word_set = set(wordDict)
        n = len(s)
        max_len = max((len(w) for w in word_set), default=0)

        # prune: can[i] = whether s[i:] is breakable at all
        can = [False] * (n + 1)
        can[n] = True
        for i in range(n - 1, -1, -1):
            end = min(n, i + max_len)
            for j in range(i + 1, end + 1):
                if can[j] and s[i:j] in word_set:
                    can[i] = True
                    break

        @lru_cache(None)
        def dfs(i: int) -> list[str]:
            if not can[i]:
                return []
            if i == n:
                return [""]

            res = []
            end = min(n, i + max_len)
            for j in range(i + 1, end + 1):
                w = s[i:j]
                if w in word_set and can[j]:
                    for tail in dfs(j):
                        res.append(w if tail == "" else w + " " + tail)
            return res

        return dfs(0)
