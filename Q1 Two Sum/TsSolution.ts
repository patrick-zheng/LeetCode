// Problem: https://leetcode.com/problems/two-sum/
// Solution: https://leetcode.com/problems/two-sum/solutions/2958/two-sum/
// Time Complexity: Brute Force O(n^2), Sorted Array with Two Pointers O(n log n), Hash Map Lookup O(n)
// Space Complexity: O(n)

type TwoSumMethod = (nums: number[], target: number) => number[];

class Solution {
    static TwoSum = class {
        static bruteForce(nums: number[], target: number): number[] {
            for (let i = 0; i < nums.length - 1; i++) {
                for (let j = i + 1; j < nums.length; j++) {
                    if (nums[i] + nums[j] === target) {
                        return [i, j];
                    }
                }
            }
            return [];
        }

        static sortedArrayTwoPointers(nums: number[], target: number): number[] {
            const indexedNums: [number, number][] = nums.map((num, i) => [i, num]);
            indexedNums.sort((a, b) => a[1] - b[1]);

            let left = 0;
            let right = indexedNums.length - 1;

            while (left < right) {
                const sum = indexedNums[left][1] + indexedNums[right][1];
                if (sum === target) {
                    return [indexedNums[left][0], indexedNums[right][0]];
                }
                else if (sum > target) { right--; }
                else { left++; }
            }

            return [];
        }

        static hashMapLookup(nums: number[], target: number): number[] {
            const numToIndex = new Map<number, number>();

            for (let i = 0; i < nums.length; i++) {
                const complement = target - nums[i];
                if (numToIndex.has(complement)) {
                    return [numToIndex.get(complement)!, i];
                }
                numToIndex.set(nums[i], i);
            }

            return [];
        }
    };

    static solve(nums: number[], target: number, method: "bruteForce" | "sortedArrayTwoPointers" | "hashMapLookup" = "hashMapLookup"): number[] {
        const methods: Record<string, TwoSumMethod> = {
            bruteForce: Solution.TwoSum.bruteForce,
            sortedArrayTwoPointers: Solution.TwoSum.sortedArrayTwoPointers,
            hashMapLookup: Solution.TwoSum.hashMapLookup,
        };

        return methods[method](nums, target);
    }
}