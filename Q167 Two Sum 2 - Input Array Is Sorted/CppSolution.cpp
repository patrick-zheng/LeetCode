#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<int> twoSum(vector<int>& numbers, int target) {
        int l = 0, r = (int)numbers.size() - 1;
        while (l < r) {
            long long s = (long long)numbers[l] + (long long)numbers[r];
            if (s == target) return {l + 1, r + 1}; // 1-indexed
            if (s < target) ++l;
            else --r;
        }
        return {}; // unreachable (guaranteed one solution)
    }
};