class Solution:
    def subsets(self, nums: list[int]) -> list[list[int]]:
        result = []
        
        def backtrack(index: int, current: list[int]):
            if index == len(nums):
                result.append(current[:])  # add a copy of current
                return
            
            # Exclude nums[index]
            backtrack(index + 1, current)
            
            # Include nums[index]
            current.append(nums[index])
            backtrack(index + 1, current)
            current.pop()
        
        backtrack(0, [])
        return result
    