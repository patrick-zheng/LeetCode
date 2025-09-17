from typing import List

class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        res = []
        path = []

        def dfs(start: int):
            # if path has enough elements, store a copy
            if len(path) == k:
                res.append(path[:])
                return
            
            # pruning: ensure enough numbers remain
            need = k - len(path)
            last_start = n - need + 1

            for i in range(start, last_start + 1):
                path.append(i)
                dfs(i + 1)
                path.pop()

        dfs(1)
        return res
    