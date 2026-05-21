#include <vector>

using namespace std;

class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        long long xor_all = 0;
        for (int x : nums) xor_all ^= x;
        long long bit = xor_all & -xor_all;
        int a = 0, b = 0;
        for (int x : nums) {
            if (x & bit) {
                a ^= x;
            } else {
                b ^= x;
            }
        }
        return {a, b};
    }
};
