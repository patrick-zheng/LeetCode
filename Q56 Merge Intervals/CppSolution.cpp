#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        if (intervals.empty()) return {};

        // Sort by start
        sort(intervals.begin(), intervals.end());

        vector<vector<int>> merged;
        int start = intervals[0][0];
        int end = intervals[0][1];

        for (int i = 1; i < intervals.size(); i++) {
            int currStart = intervals[i][0];
            int currEnd = intervals[i][1];

            if (currStart <= end) {
                // Overlap → extend
                end = max(end, currEnd);
            } else {
                merged.push_back({start, end});
                start = currStart;
                end = currEnd;
            }
        }
        merged.push_back({start, end});

        return merged;
    }
};
