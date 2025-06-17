"""
Problem: https://leetcode.com/problems/two-sum/
Solution: https://leetcode.com/problems/two-sum/solutions/2958/two-sum/
Time Complexity: Brute Force O(n^2), Sorted Array with Two Pointers O(n log n), Hash Map Lookup O(n)
Space Complexity: O(n)
"""

from typing import Callable

class Solution:
    class TwoSum:
        @staticmethod
        def bruteForce(nums: list[int], target: int) -> list[int]:
            for i in range(len(nums) - 1):
                for j in range(i + 1, len(nums)):
                    if (nums[i] + nums[j]) == target:
                        return [i, j]
            return []

        @staticmethod
        def sortedArrayTwoPointers(nums: list[int], target: int) -> list[int]:
            zippedNumsMap = [(i, num) for i, num in enumerate(nums)]
            sortedMap = sorted(zippedNumsMap, key=lambda x: x[1])

            leftIdx = 0
            rightIdx = len(sortedMap) - 1

            while leftIdx < rightIdx:
                trySum = sortedMap[leftIdx][1] + sortedMap[rightIdx][1]
                if trySum == target:
                    return [sortedMap[leftIdx][0], sortedMap[rightIdx][0]]
                elif trySum > target: rightIdx -= 1
                else: leftIdx += 1

            return []

        @staticmethod
        def hashMapLookup(nums: list[int], target: int) -> list[int]:
            numIdxMap = {}

            for i, num in enumerate(nums):
                complement = target - num
                if complement in numIdxMap:
                    return [numIdxMap[complement], i]
                if num not in numIdxMap:
                    numIdxMap[num] = i

            return []

    @staticmethod
    def solve(nums: list[int], target: int, method: str = "hashMapLookup") -> list[int]:
        methods: dict[str, Callable[[list[int], int], list[int]]] = {
            "bruteForce": Solution.TwoSum.bruteForce,
            "sortedArrayTwoPointers": Solution.TwoSum.sortedArrayTwoPointers,
            "hashMapLookup": Solution.TwoSum.hashMapLookup
        }
        return methods[method](nums, target)
