"""
Problem: https://leetcode.com/problems/two-sum/
Solution: https://leetcode.com/problems/two-sum/solutions/2958/two-sum/
Time Complexity: Brute Force O(n^2), Sorted Array with Two Pointers O(n log n), Hash Map Lookup O(n)
Space Complexity: O(n)
"""

from __future__ import annotations
from typing import Callable

class Solution:
    @staticmethod
    def solve(nums: list[int], target: int, method: str = "hashMapLookup") -> list[int]:
        methods: dict[str, Callable[[list[int], int], list[int]]] = {
            "bruteForce": Solution.twoSum.bruteForce,
            "sortedArrayTwoPointers": Solution.twoSum.sortedArrayTwoPointers,
            "hashMapLookup": Solution.twoSum.hashMapLookup
        }
        return methods[method](nums, target)

    @staticmethod
    def twoSum(nums: list[int], target: int) -> list[int]:
            def bruteForce(nums: list[int], target: int) -> list[int]:
                for i in range(len(nums) - 1):
                    for j in range(i + 1, len(nums)):
                        if (nums[i] + nums[j]) == target:
                            return [i, j]

            def sortedArrayTwoPointers(nums: list[int], target: int) -> list[int]:
                zippedNumsMap = [(i, num) for i, num in enumerate(nums)]
                sortedMap = sorted(zippedNumsMap, key=lambda x: x[1])

                leftIdx = 0
                rightIdx = len(sortedMap) - 1

                while True:
                    leftNum = sortedMap[leftIdx][1]
                    rightNum = sortedMap[rightIdx][1]
                    trySum = leftNum + rightNum

                    if trySum == target: return [sortedMap[leftIdx][0], sortedMap[rightIdx][0]]
                    elif trySum > target: rightIdx -= 1
                    else: leftIdx += 1

            def hashMapLookup(nums: list[int], target: int) -> list[int]:
                numIdxMap = {}
                for i, num in enumerate(nums):
                    complement = target - num
                    if complement in numIdxMap: return [numIdxMap[complement], i]
                    numIdxMap[num] = i
