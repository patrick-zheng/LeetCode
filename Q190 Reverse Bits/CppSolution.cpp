#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int reverseBits(int n) {
        uint32_t res = 0;
        for (int i = 0; i < 32; ++i) {
            res = (res << 1) | (n & 1u);
            n >>= 1;
        }
        return res;
    }
};
