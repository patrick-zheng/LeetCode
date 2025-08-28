#include <vector>
using namespace std;

class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        int m = obstacleGrid.size(), n = obstacleGrid[0].size();
        if (obstacleGrid[0][0] == 1 || obstacleGrid[m-1][n-1] == 1) return 0;

        vector<long long> dp(n, 0);
        dp[0] = 1;

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (obstacleGrid[i][j] == 1) {
                    dp[j] = 0;                    // obstacle blocks this cell
                } else if (j > 0) {
                    dp[j] += dp[j - 1];           // from left + from top (in dp[j])
                }
            }
        }
        return static_cast<int>(dp[n - 1]);
    }
};
