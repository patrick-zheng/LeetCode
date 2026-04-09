#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<vector<int>> res;
        vector<int> path;
        path.reserve(static_cast<size_t>(k));
        dfs(1, k, n, path, res);
        return res;
    }

private:
    static int minSumK(int start, int kLeft) {
        return kLeft * start + kLeft * (kLeft - 1) / 2;
    }

    static int maxSumK(int start, int kLeft) {
        int total = 0;
        int x = 9;
        for (int t = 0; t < kLeft; ++t) {
            if (x < start) return -1;
            total += x;
            --x;
        }
        return total;
    }

    void dfs(int start, int kLeft, int target, vector<int>& path, vector<vector<int>>& res) {
        if (kLeft == 0) {
            if (target == 0) res.push_back(path);
            return;
        }
        if (start > 9 || target <= 0) return;
        if (10 - start < kLeft) return;

        int lo = minSumK(start, kLeft);
        int hi = maxSumK(start, kLeft);
        if (hi < 0 || target < lo || target > hi) return;

        for (int i = start; i <= 9; ++i) {
            if (i > target) break;
            path.push_back(i);
            dfs(i + 1, kLeft - 1, target - i, path, res);
            path.pop_back();
        }
    }
};
