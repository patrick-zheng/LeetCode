#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> ans;
        vector<int> path;
        dfs(1, n, k, path, ans);
        return ans;
    }

private:
    void dfs(int start, int n, int k, vector<int>& path, vector<vector<int>>& ans) {
        if ((int)path.size() == k) {
            ans.push_back(path);
            return;
        }
        // Numbers remaining to pick:
        int need = k - (int)path.size();
        // Last valid start to ensure enough numbers remain:
        int lastStart = n - need + 1;

        for (int i = start; i <= lastStart; ++i) {
            path.push_back(i);
            dfs(i + 1, n, k, path, ans);
            path.pop_back();
        }
    }
};
