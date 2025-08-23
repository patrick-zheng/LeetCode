#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    string getPermutation(int n, int k) {
        vector<int> nums;
        nums.reserve(n);
        for (int i = 1; i <= n; ++i) nums.push_back(i);

        vector<int> fact(n + 1, 1);
        for (int i = 1; i <= n; ++i) fact[i] = fact[i - 1] * i;

        k--;
        string res;
        res.reserve(n);

        for (int i = n; i >= 1; --i) {
            int block = fact[i - 1];
            int idx = k / block;
            res += to_string(nums[idx]);
            nums.erase(nums.begin() + idx);
            k %= block;
        }
        return res;
    }
};
