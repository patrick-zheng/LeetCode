#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int mySqrt(int x) {
        if (x < 2) return x;

        int lo = 1, hi = x / 2, ans = 0;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            if (mid <= x / mid) { // avoid mid*mid overflow
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        return ans;
    }
};
