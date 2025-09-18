#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> result;
        vector<int> current;
        backtrack(0, nums, current, result);
        return result;
    }

private:
    void backtrack(int index, vector<int>& nums, vector<int>& current, vector<vector<int>>& result) {
        if (index == nums.size()) {
            result.push_back(current);
            return;
        }

        // Exclude nums[index]
        backtrack(index + 1, nums, current, result);

        // Include nums[index]
        current.push_back(nums[index]);
        backtrack(index + 1, nums, current, result);
        current.pop_back();
    }
};
