class Solution:
    def combinationSum(self, candidates: list[int], target: int) -> list[list[int]]:
        def backtrack(start, path, remaining):
            if remaining == 0:
                result.append(path)
                return
            if remaining < 0:
                return
            
            for i in range(start, len(candidates)):
                backtrack(i, path + [candidates[i]], remaining - candidates[i])
        
        result = []
        backtrack(0, [], target)
        return result
    