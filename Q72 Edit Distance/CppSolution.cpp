#include <string>
#include <vector>
#include <algorithm>

class Solution {
public:
    int minDistance(const std::string& word1, const std::string& word2) {
        // Keep 'a' as the longer string to minimize memory.
        auto const* a = &word1;
        auto const* b = &word2;
        if (a->size() < b->size()) std::swap(a, b);

        const int m = static_cast<int>(a->size());
        const int n = static_cast<int>(b->size());

        // dp[j] = distance between a[:i] and b[:j] for current i
        std::vector<int> dp(n + 1);
        for (int j = 0; j <= n; ++j) dp[j] = j; // base: convert "" -> b[:j] by inserting j chars

        for (int i = 1; i <= m; ++i) {
            int prev_diag = dp[0]; // old dp[j-1] from previous row (top-left cell)
            dp[0] = i;             // base: convert a[:i] -> "" by deleting i chars

            for (int j = 1; j <= n; ++j) {
                int top = dp[j]; // old dp[i-1][j]
                if ((*a)[i - 1] == (*b)[j - 1]) {
                    dp[j] = prev_diag; // no operation needed
                } else {
                    int insert_cost  = dp[j - 1] + 1; // left  + 1
                    int delete_cost  = top + 1;       // top   + 1
                    int replace_cost = prev_diag + 1; // diag  + 1
                    dp[j] = std::min({insert_cost, delete_cost, replace_cost});
                }
                prev_diag = top; // advance diagonal for next j
            }
        }
        return dp[n];
    }
};