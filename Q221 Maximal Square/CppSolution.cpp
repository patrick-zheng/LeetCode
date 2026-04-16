#include <algorithm>
#include <vector>

using namespace std;

class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        if (matrix.empty() || matrix[0].empty()) return 0;
        int rows = static_cast<int>(matrix.size());
        int cols = static_cast<int>(matrix[0].size());
        vector<int> dp(cols, 0);
        int bestSide = 0;

        for (int i = 0; i < rows; ++i) {
            int northwest = 0;
            for (int j = 0; j < cols; ++j) {
                int above = dp[j];
                if (matrix[i][j] == '1') {
                    int left = (j > 0) ? dp[j - 1] : 0;
                    dp[j] = min({above, left, northwest}) + 1;
                    bestSide = max(bestSide, dp[j]);
                } else {
                    dp[j] = 0;
                }
                northwest = above;
            }
        }
        return bestSide * bestSide;
    }
};
