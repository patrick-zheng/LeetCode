#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        vector<vector<int>> res;
        int i = 0, n = intervals.size();
        int s = newInterval[0], e = newInterval[1];

        // 1) Add intervals that end before newInterval starts
        while (i < n && intervals[i][1] < s) {
            res.push_back(intervals[i++]);
        }

        // 2) Merge all overlapping intervals
        while (i < n && intervals[i][0] <= e) {
            s = min(s, intervals[i][0]);
            e = max(e, intervals[i][1]);
            ++i;
        }
        res.push_back({s, e});

        // 3) Add the remaining intervals
        while (i < n) {
            res.push_back(intervals[i++]);
        }

        return res;
    }
};
