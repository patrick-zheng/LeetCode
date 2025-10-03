#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<int> grayCode(int n) {
        vector<int> res;
        int size = 1 << n; // 2^n
        res.reserve(size);
        for (int i = 0; i < size; i++) {
            res.push_back(i ^ (i >> 1));
        }
        return res;
    }
};
