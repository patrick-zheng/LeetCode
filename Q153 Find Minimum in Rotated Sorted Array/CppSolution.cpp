#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int findMin(vector<int>& nums) {
        int l = 0, r = (int)nums.size() - 1;

        while (l < r) {
            int mid = l + (r - l) / 2;

            if (nums[mid] > nums[r]) {
                // Minimum must be to the right of mid
                l = mid + 1;
            } else {
                // Minimum is at mid or to the left
                r = mid;
            }
        }
        return nums[l];
    }
};
