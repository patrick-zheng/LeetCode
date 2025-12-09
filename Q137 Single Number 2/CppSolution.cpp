#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int singleNumber(vector<int>& nums) {
        int ones = 0;  // bits that have appeared once
        int twos = 0;  // bits that have appeared twice

        for (int x : nums) {
            ones = (ones ^ x) & ~twos;
            twos = (twos ^ x) & ~ones;
        }

        return ones;   // bits that have appeared 1 mod 3 times
    }
};
