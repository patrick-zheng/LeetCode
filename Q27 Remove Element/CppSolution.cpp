#include <vector>

using namespace std;

class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int n = nums.size();
        int i = 0;
        while (i < n) {
            if (nums[i] == val) {
                nums[i] = nums [n - 1];
                n -= 1;
            } else { i += 1; }
        }
        return n;
    }
};
