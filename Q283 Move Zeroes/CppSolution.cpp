#include <vector>

using namespace std;

class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int write = 0;
        for (int i = 0; i < static_cast<int>(nums.size()); ++i) {
            if (nums[i] != 0) {
                if (i != write) {
                    swap(nums[write], nums[i]);
                }
                ++write;
            }
        }
    }
};
