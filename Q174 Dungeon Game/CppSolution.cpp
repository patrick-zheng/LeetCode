#include <vector>
#include <algorithm>
#include <climits>

using namespace std;

class Solution {
public:
    int calculateMinimumHP(vector<vector<int>>& dungeon) {
        int m = (int)dungeon.size();
        int n = (int)dungeon[0].size();
        const long long INF = (1LL << 60);

        vector<long long> dp(n + 1, INF);
        dp[n - 1] = 1; // sentinel

        for (int i = m - 1; i >= 0; --i) {
            dp[n] = INF; // right boundary sentinel for this row
            for (int j = n - 1; j >= 0; --j) {
                long long need_next = min(dp[j], dp[j + 1]);
                long long need_here = need_next - (long long)dungeon[i][j];
                dp[j] = (need_here <= 1) ? 1 : need_here;
            }
        }
        return (int)dp[0];
    }
};
