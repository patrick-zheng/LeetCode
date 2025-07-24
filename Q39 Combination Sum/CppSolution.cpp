#include <vector>
#include <string>
#include <functional>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> result;

        function<void(int, vector<int>&, int)> backtrack = [&](int start, vector<int>& path, int remaining) {
            if (remaining == 0) {
                result.push_back(path);
                return;
            }
            if (remaining < 0) return;

            for (int i = start; i < candidates.size(); ++i) {
                path.push_back(candidates[i]);
                backtrack(i, path, remaining - candidates[i]);
                path.pop_back(); // backtrack
            }
        };

        vector<int> path;
        backtrack(0, path, target);
        return result;
    }
};