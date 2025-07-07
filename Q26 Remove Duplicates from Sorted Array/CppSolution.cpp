#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int write = 0;
        for (int read : nums) {
            if (write == 0 || read != nums[write - 1]) {
                nums[write] = read;
                ++write;
            }
        }
        return write;
    }
};
