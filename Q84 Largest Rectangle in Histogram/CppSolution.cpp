#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    int largestRectangleArea(vector<int>& heights) {
        vector<int> h = heights;
        h.push_back(0);
        vector<int> st;
        long long best = 0;

        for (int i = 0; i < (int)h.size(); ++i) {
            while (!st.empty() && h[st.back()] > h[i]) {
                int top = st.back(); st.pop_back();
                long long height = h[top];
                int leftIdx = st.empty() ? -1 : st.back();
                long long width = i - leftIdx - 1;
                best = max(best, height * width);
            }
            st.push_back(i);
        }
        return (int)best;
    }
};
