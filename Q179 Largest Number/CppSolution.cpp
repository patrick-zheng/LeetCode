#include <vector>
#include <string>
#include <algorithm>

using namespace std;

class Solution {
public:
    string largestNumber(vector<int>& nums) {
        vector<string> strs;
        strs.reserve(nums.size());

        for (int num : nums) {
            strs.push_back(to_string(num));
        }

        sort(strs.begin(), strs.end(),
            [](const string& a, const string& b) {
                return a + b > b + a;   // descending by concatenation
            }
        );

        // Handle case like [0,0]
        if (!strs.empty() && strs[0] == "0") {
            return "0";
        }

        string result;
        for (const auto& s : strs) {
            result += s;
        }

        return result;
    }
};
