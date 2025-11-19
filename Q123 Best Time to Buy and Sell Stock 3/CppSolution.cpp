#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.empty()) return 0;

        int buy1  = numeric_limits<int>::min();
        int sell1 = 0;
        int buy2  = numeric_limits<int>::min();
        int sell2 = 0;

        for (int price : prices) {
            sell2 = max(sell2, buy2 + price);   // second sell
            buy2  = max(buy2, sell1 - price);   // second buy
            sell1 = max(sell1, buy1 + price);   // first sell
            buy1  = max(buy1, -price);          // first buy
        }

        return sell2;
    }
};