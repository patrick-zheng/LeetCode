#include <vector>
#include <climits>
#include <algorithm>

using namespace std;

class Solution {
public:
    int maximumGap(vector<int>& nums) {
        int n = (int)nums.size();
        if (n < 2) return 0;

        int mn = INT_MAX, mx = INT_MIN;
        for (int x : nums) {
            mn = min(mn, x);
            mx = max(mx, x);
        }
        if (mn == mx) return 0;

        // bucketSize = ceil((mx - mn) / (n - 1))
        long long range = (long long)mx - (long long)mn;
        int bucketCount = n - 1;
        long long bucketSize = (range + bucketCount - 1) / bucketCount; // ceil
        bucketSize = max(1LL, bucketSize);

        vector<int> bmin(bucketCount, INT_MAX);
        vector<int> bmax(bucketCount, INT_MIN);
        vector<char> used(bucketCount, 0);

        for (int x : nums) {
            if (x == mn || x == mx) continue;
            int idx = (int)(((long long)x - mn) / bucketSize);
            bmin[idx] = min(bmin[idx], x);
            bmax[idx] = max(bmax[idx], x);
            used[idx] = 1;
        }

        int ans = 0;
        int prev = mn;

        for (int i = 0; i < bucketCount; i++) {
            if (!used[i]) continue;
            ans = max(ans, bmin[i] - prev);
            prev = bmax[i];
        }

        ans = max(ans, mx - prev);
        return ans;
    }
};
