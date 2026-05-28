#include <vector>

using namespace std;

class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int missing = nums.size();
        for (int i = 0; i < static_cast<int>(nums.size()); ++i) {
            missing ^= i ^ nums[i];
        }
        return missing;
    }
};
