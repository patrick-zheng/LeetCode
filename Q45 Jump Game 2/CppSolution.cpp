#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int jump(vector<int>& nums) {
        int n = nums.size();
        if (n <= 1) return 0;

        int jumps = 0, farthest = 0, currentEnd = 0;

        for (int i = 0; i < n - 1; ++i) {
            farthest = max(farthest, i + nums[i]);
            if (i == currentEnd) {
                ++jumps;
                currentEnd = farthest;
            }
        }

        return jumps;
    }
};
