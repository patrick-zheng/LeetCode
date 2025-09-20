#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int w = 0;
        for (int x : nums) {
            if (w < 2 || x != nums[w - 2]) {
                nums[w] = x;
                ++w;
            }
        }
        return w;
    }
};
