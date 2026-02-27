#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int maxProfit(int k, vector<int>& prices) {
        int n = static_cast<int>(prices.size());
        if (n == 0 || k == 0) return 0;

        // Unlimited transactions case
        if (k >= n / 2) {
            int profit = 0;
            for (int i = 1; i < n; ++i) {
                if (prices[i] > prices[i - 1])
                    profit += prices[i] - prices[i - 1];
            }
            return profit;
        }

        vector<int> prev(n, 0);
        vector<int> cur(n, 0);

        for (int t = 1; t <= k; ++t) {
            int best = -prices[0];  // prev[0] - prices[0]
            for (int i = 1; i < n; ++i) {
                cur[i] = max(cur[i - 1], prices[i] + best);
                best = max(best, prev[i] - prices[i]);
            }
            swap(prev, cur);
        }

        return prev[n - 1];
    }
};