#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    void sortColors(vector<int>& nums) {
        int low = 0, mid = 0, high = static_cast<int>(nums.size()) - 1;

        while (mid <= high) {
            if (nums[mid] == 0) {
                std::swap(nums[low++], nums[mid++]);
            } else if (nums[mid] == 1) {
                ++mid;
            } else { // nums[mid] == 2
                std::swap(nums[mid], nums[high--]);
            }
        }
    }
};