#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.size() < 2) return 0;

        int min_price = prices[0];
        int max_profit = 0;

        for (size_t i = 1; i < prices.size(); ++i) {
            int price = prices[i];

            // Profit if we sell today
            int profit = price - min_price;
            if (profit > max_profit) {
                max_profit = profit;
            }

            // Update minimum price seen so far
            if (price < min_price) {
                min_price = price;
            }
        }

        return max_profit;
    }
};
