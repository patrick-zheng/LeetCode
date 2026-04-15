#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
    static long long bucket_id(long long num, long long width) {
        return num >= 0 ? num / width : (num + 1) / width - 1;
    }

public:
    bool containsNearbyAlmostDuplicate(vector<int>& nums, int k, int t) {
        if (k < 0 || t < 0) return false;
        const long long width = static_cast<long long>(t) + 1;
        unordered_map<long long, long long> buckets;
        const int n = static_cast<int>(nums.size());
        for (int i = 0; i < n; ++i) {
            if (i > k) {
                const long long left = nums[i - k - 1];
                buckets.erase(bucket_id(left, width));
            }
            const long long num = nums[i];
            const long long b = bucket_id(num, width);
            if (buckets.count(b)) return true;
            auto it = buckets.find(b - 1);
            if (it != buckets.end() && num - it->second <= t) return true;
            it = buckets.find(b + 1);
            if (it != buckets.end() && it->second - num <= t) return true;
            buckets[b] = num;
        }
        return false;
    }
};
