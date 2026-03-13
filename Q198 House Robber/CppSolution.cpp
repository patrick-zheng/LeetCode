#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int rob(vector<int>& nums) {
        int prev2 = 0, prev1 = 0;

        for (int money : nums) {
            int current = max(prev1, prev2 + money);
            prev2 = prev1;
            prev1 = current;
        }

        return prev1;
    }
};
