#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        unordered_set<int> window;
        window.reserve(min(static_cast<int>(nums.size()), k + 1));
        const int n = static_cast<int>(nums.size());
        for (int i = 0; i < n; ++i) {
            if (!window.insert(nums[i]).second) return true;
            if (i >= k) window.erase(nums[i - k]);
        }
        return false;
    }
};
