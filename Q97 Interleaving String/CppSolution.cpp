#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool isInterleave(string s1, string s2, string s3) {
        int n = s1.size(), m = s2.size();
        if (n + m != (int)s3.size()) return false;

        // Ensure s2 is the shorter to minimize space
        if (m > n) { swap(s1, s2); swap(n, m); }

        vector<char> dp(m + 1, false);
        dp[0] = true;

        // Initialize first row using only s2
        for (int j = 1; j <= m; ++j) {
            dp[j] = dp[j - 1] && (s2[j - 1] == s3[j - 1]);
        }

        for (int i = 1; i <= n; ++i) {
            // First column using only s1
            dp[0] = dp[0] && (s1[i - 1] == s3[i - 1]);
            for (int j = 1; j <= m; ++j) {
                bool take_s1 = dp[j] && (s1[i - 1] == s3[i + j - 1]);
                bool take_s2 = dp[j - 1] && (s2[j - 1] == s3[i + j - 1]);
                dp[j] = (take_s1 || take_s2);
            }
        }
        return dp[m];
    }
};