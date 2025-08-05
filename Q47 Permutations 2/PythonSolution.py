class Solution:
    def permuteUnique(self, nums: list[int]) -> list[list[int]]:
        from itertools import permutations
        perms = set(permutations(nums))
        return [list(p) for p in perms]
