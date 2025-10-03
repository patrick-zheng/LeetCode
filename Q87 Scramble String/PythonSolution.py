from functools import lru_cache


class Solution:
    def isScramble(self, s1: str, s2: str) -> bool:
        if len(s1) != len(s2):
            return False
        n = len(s1)
        if s1 == s2:
            return True

        # Build 26-letter prefix counts so we can compare any substring in O(26)
        def build_prefix(s: str):
            pref = [[0]*26 for _ in range(n+1)]
            for i, ch in enumerate(s, 1):
                row = pref[i-1].copy()    # copy 26 ints
                row[ord(ch) - 97] += 1
                pref[i] = row
            return pref

        pref1 = build_prefix(s1)
        pref2 = build_prefix(s2)

        def same_multiset(p1: int, p2: int, length: int) -> bool:
            # Compare counts of s1[p1:p1+length] vs s2[p2:p2+length]
            a = pref1[p1 + length]
            b = pref1[p1]
            c = pref2[p2 + length]
            d = pref2[p2]
            # Compare (a - b) vs (c - d) component-wise
            for k in range(26):
                if (a[k] - b[k]) != (c[k] - d[k]):
                    return False
            return True

        @lru_cache(maxsize=None)
        def dfs(i: int, j: int, length: int) -> bool:
            # Exact match short-circuit
            if s1[i:i+length] == s2[j:j+length]:
                return True

            # Multiset pruning: if letters differ, impossible
            if not same_multiset(i, j, length):
                return False

            # Try all split points
            for k in range(1, length):
                # Case 1: no swap
                if dfs(i, j, k) and dfs(i + k, j + k, length - k):
                    return True
                # Case 2: swap
                if dfs(i, j + length - k, k) and dfs(i + k, j, length - k):
                    return True

            return False

        return dfs(0, 0, n)
