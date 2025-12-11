#include <vector>
#include <string>
#include <unordered_set>
#include <algorithm>

using namespace std;

class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        unordered_set<string> wordSet(wordDict.begin(), wordDict.end());
        int n = s.size();
        vector<bool> dp(n + 1, false);
        dp[0] = true;

        int maxLen = 0;
        for (const auto& w : wordDict) {
            if ((int)w.size() > maxLen) maxLen = w.size();
        }

        for (int i = 1; i <= n; ++i) {
            int start = max(0, i - maxLen);
            for (int j = start; j < i; ++j) {
                if (dp[j] && wordSet.count(s.substr(j, i - j))) {
                    dp[i] = true;
                    break;
                }
            }
        }

        return dp[n];
    }
};
