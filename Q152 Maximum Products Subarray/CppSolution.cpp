#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int maxEnding = nums[0];
        int minEnding = nums[0];
        int best = nums[0];

        for (int i = 1; i < (int)nums.size(); i++) {
            int x = nums[i];

            // Negative flips roles of max/min
            if (x < 0) swap(maxEnding, minEnding);

            maxEnding = max(x, x * maxEnding);
            minEnding = min(x, x * minEnding);

            best = max(best, maxEnding);
        }

        return best;
    }
};
