#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int trailingZeroes(int n) {
        long long x = n;
        long long ans = 0;
        while (x > 0) {
            x /= 5;
            ans += x;
        }
        return (int)ans;
    }
};
