class Solution:
    def permute(self, nums: list[int]) -> list[list[int]]:
        from itertools import permutations
        return [list(p) for p in permutations(nums)]
