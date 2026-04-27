#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        vector<string> ranges;
        if (nums.empty()) {
            return ranges;
        }

        int start = nums[0];
        int prev = nums[0];

        for (int i = 1; i < static_cast<int>(nums.size()); ++i) {
            if (nums[i] == prev + 1) {
                prev = nums[i];
                continue;
            }

            if (start == prev) {
                ranges.push_back(to_string(start));
            } else {
                ranges.push_back(to_string(start) + "->" + to_string(prev));
            }

            start = nums[i];
            prev = nums[i];
        }

        if (start == prev) {
            ranges.push_back(to_string(start));
        } else {
            ranges.push_back(to_string(start) + "->" + to_string(prev));
        }

        return ranges;
    }
};
