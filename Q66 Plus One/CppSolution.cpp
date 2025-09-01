#include <vector>

using namespace std;

class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int carry = 1; // we add one
        for (int i = (int)digits.size() - 1; i >= 0 && carry; --i) {
            int total = digits[i] + carry;
            digits[i] = total % 10;
            carry = total / 10;
        }
        if (carry) digits.insert(digits.begin(), carry);
        return digits;
    }
};
