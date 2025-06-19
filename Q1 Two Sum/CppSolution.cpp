#include <vector>
#include <unordered_map>
#include <algorithm>
#include <string>
#include <stdexcept>

/*
 * Problem: https://leetcode.com/problems/two-sum/
 * Solution: https://leetcode.com/problems/two-sum/solutions/
 * Time Complexity: Brute Force O(n^2), Sorted Array with Two Pointers O(n log n), Hash Map Lookup O(n)
 * Space Complexity: O(n)
 */

 using namespace std;

class Solution {
public:
    class TwoSum {
    public:
        static vector<int> bruteForce(const vector<int>& nums, int target) {
            for (size_t i = 0; i < nums.size() - 1; ++i) {
                for (size_t j = i + 1; j < nums.size(); ++j) {
                    if (nums[i] + nums[j] == target) {
                        return {static_cast<int>(i), static_cast<int>(j)};
                    }
                }
            }
            return {};
        }

        static vector<int> sortedArrayTwoPointers(const vector<int>& nums, int target) {
            vector<pair<int, int>> idxNumsMap;
            for (int i = 0; i < nums.size(); ++i) {
                idxNumsMap.emplace_back(nums[i], i);
            }

            sort(idxNumsMap.begin(), idxNumsMap.end());

            int left = 0;
            int right = static_cast<int>(idxNumsMap.size()) - 1;

            while (left < right) {
                int sum = idxNumsMap[left].first + idxNumsMap[right].first;
                if (sum == target) {
                    return {idxNumsMap[left].second, idxNumsMap[right].second};
                }
                else if (sum > target) { --right; }
                else { ++left; }
            }

            return {};
        }

        static vector<int> hashMapLookup(const vector<int>& nums, int target) {
            unordered_map<int, int> numIdxMap;

            for (int i = 0; i < nums.size(); ++i) {
                int complement = target - nums[i];
                if (numIdxMap.find(complement) != numIdxMap.end()) {
                    return {numIdxMap[complement], i};
                }
                if (numIdxMap.find(nums[i]) == numIdxMap.end()) {
                    numIdxMap[nums[i]] = i;
                }
            }

            return {};
        }
    };

    static vector<int> solve(const vector<int>& nums, int target, const string& method = "hashMapLookup") {
        if (method == "bruteForce") {
            return TwoSum::bruteForce(nums, target);
        } else if (method == "sortedArrayTwoPointers") {
            return TwoSum::sortedArrayTwoPointers(nums, target);
        } else if (method == "hashMapLookup") {
            return TwoSum::hashMapLookup(nums, target);
        } else {
            throw invalid_argument("Unknown method: " + method);
        }
    }
};
