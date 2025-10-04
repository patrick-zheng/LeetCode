#include <algorithm>
#include <vector>
#include <functional>

using namespace std;

class Solution {
public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> res;
        vector<int> path;

        function<void(size_t)> dfs = [&](size_t start) {
            res.emplace_back(path);
            for (size_t i = start; i < nums.size(); ++i) {
                if (i > start && nums[i] == nums[i - 1]) continue; // skip dups
                path.push_back(nums[i]);
                dfs(i + 1);
                path.pop_back();
            }
        };

        dfs(0);
        return res;
    }
};
