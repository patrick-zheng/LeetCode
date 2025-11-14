#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        int n = triangle.size();
        vector<int> dp = triangle[n - 1];  // start from the last row

        // Move upwards
        for (int r = n - 2; r >= 0; --r) {
            for (int c = 0; c < (int)triangle[r].size(); ++c) {
                dp[c] = triangle[r][c] + min(dp[c], dp[c + 1]);
            }
        }
        return dp[0];
    }
};
