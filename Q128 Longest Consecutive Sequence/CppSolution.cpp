#include <vector>
#include <unordered_set>

using namespace std;

class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        if (nums.empty()) return 0;

    unordered_set<int> s(nums.begin(), nums.end());
    int longest = 0;

    for (int x : s) {
        // start of a sequence
        if (s.find(x - 1) == s.end()) {
            int current = x;
            int length = 1;

            while (s.find(current + 1) != s.end()) {
                current++;
                length++;
            }

            if (length > longest) {
                longest = length;
            }
        }
    }

    return longest;
    }
};
