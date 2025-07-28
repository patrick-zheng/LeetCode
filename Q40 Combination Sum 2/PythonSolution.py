class Solution:
    def combinationSum2(self, candidates: list[int], target: int) -> list[list[int]]:
        candidates.sort()  # Sort to handle duplicates
        res = []

        def backtrack(start, path, total):
            if total == target:
                res.append(path[:])
                return
            if total > target:
                return

            prev = -1
            for i in range(start, len(candidates)):
                curr = candidates[i]
                # Skip duplicates
                if curr == prev:
                    continue
                # Early stopping (pruning)
                if total + curr > target:
                    break
                path.append(curr)
                backtrack(i + 1, path, total + curr)
                path.pop()
                prev = curr

        backtrack(0, [], 0)
        return res
    