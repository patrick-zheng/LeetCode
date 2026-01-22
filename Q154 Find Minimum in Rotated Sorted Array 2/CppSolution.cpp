#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int findMin(vector<int>& nums) {
        int l = 0, r = (int)nums.size() - 1;

        while (l < r) {
            int mid = l + (r - l) / 2;

            if (nums[mid] < nums[r]) {
                r = mid;
            } else if (nums[mid] > nums[r]) {
                l = mid + 1;
            } else {
                r--; // duplicates -> cannot decide
            }
        }
        return nums[l];
    }
};
