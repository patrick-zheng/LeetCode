#include <vector>
#include <string>
#include <algorithm>

using namespace std;

class Solution {
public:
    int minCut(string s) {
        int n = static_cast<int>(s.size());
        if (n <= 1) return 0;

        // pal[i][j] == true if s[i..j] is a palindrome
        vector<vector<bool>> pal(n, vector<bool>(n, false));

        // Fill palindrome table
        for (int i = n - 1; i >= 0; --i) {
            for (int j = i; j < n; ++j) {
                if (s[i] == s[j] && (j - i < 2 || pal[i + 1][j - 1])) {
                    pal[i][j] = true;
                }
            }
        }

        // dp[i] = min cuts for s[0..i]
        vector<int> dp(n, 0);

        for (int i = 0; i < n; ++i) {
            int minCuts = i; // worst case: cut before each char
            if (pal[0][i]) {
                minCuts = 0; // whole prefix is a palindrome
            } else {
                for (int j = 0; j < i; ++j) {
                    if (pal[j + 1][i]) {
                        minCuts = min(minCuts, dp[j] + 1);
                    }
                }
            }
            dp[i] = minCuts;
        }

        return dp[n - 1];
    }
};
