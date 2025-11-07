#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int numDistinct(string s, string t) {
        int m = (int)s.size(), n = (int)t.size();
        if (n == 0) return 1;
        if (m < n) return 0;

        using U128 = unsigned __int128;
        vector<U128> dp(n + 1, 0);
        dp[0] = 1;

        for (char ch : s) {
            for (int j = n; j >= 1; --j) {
                if (ch == t[j - 1]) {
                    dp[j] += dp[j - 1]; // modulo 2^128, no UB
                }
            }
        }
        return static_cast<int>(dp[n]); // LC guarantees fits in 32-bit
    }
};
