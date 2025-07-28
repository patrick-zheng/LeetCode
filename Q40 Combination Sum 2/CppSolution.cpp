#include <vector>
#include <algorithm>

class Solution {
public:
    std::vector<std::vector<int>> combinationSum2(std::vector<int>& candidates, int target) {
        std::sort(candidates.begin(), candidates.end());
        std::vector<std::vector<int>> res;
        std::vector<int> path;
        path.reserve(candidates.size());  // avoid reallocations
        dfs(candidates, target, 0, path, res);
        return res;
    }

private:
    void dfs(const std::vector<int>& candidates, int target, int start,
             std::vector<int>& path, std::vector<std::vector<int>>& res) {
        if (target == 0) {
            res.emplace_back(path);
            return;
        }

        for (int i = start; i < candidates.size(); ++i) {
            int curr = candidates[i];

            // Skip duplicates
            if (i > start && curr == candidates[i - 1]) continue;

            // Prune search
            if (curr > target) break;

            path.push_back(curr);
            dfs(candidates, target - curr, i + 1, path, res);
            path.pop_back();
        }
    }
};
