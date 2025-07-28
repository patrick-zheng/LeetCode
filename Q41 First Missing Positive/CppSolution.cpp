#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int n = nums.size();
        int i = 0;

        while (i < n) {
            int val = nums[i];
            if (val > 0 && val <= n && nums[i] != nums[val - 1]) {
                swap(nums[i], nums[val - 1]);
            } else {
                i++;
            }
        }

        for (int i = 0; i < n; i++) {
            if (nums[i] != i + 1) return i + 1;
        }

        return n + 1;
    }
};
