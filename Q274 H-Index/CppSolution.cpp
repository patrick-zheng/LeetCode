#include <vector>

using namespace std;

class Solution {
public:
    int hIndex(vector<int>& citations) {
        int n = citations.size();
        vector<int> bucket(n + 1, 0);
        for (int c : citations) {
            if (c >= n) {
                ++bucket[n];
            } else {
                ++bucket[c];
            }
        }

        int papers = 0;
        for (int h = n; h >= 0; --h) {
            papers += bucket[h];
            if (papers >= h) {
                return h;
            }
        }
        return 0;
    }
};
