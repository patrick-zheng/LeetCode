#include <vector>
#include <functional>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> results;
        vector<bool> used(nums.size(), false);
        vector<int> path;

        function<void()> backtrack = [&]() {
            if (path.size() == nums.size()) {
                results.push_back(path);
                return;
            }

            for (int i = 0; i < nums.size(); ++i) {
                if (used[i]) continue;
                if (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) continue;

                used[i] = true;
                path.push_back(nums[i]);
                backtrack();
                path.pop_back();
                used[i] = false;
            }
        };

        backtrack();
        return results;
    }
};
