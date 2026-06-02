#include <vector>

using namespace std;

class Solution {
public:
    int hIndex(vector<int>& citations) {
        int n = static_cast<int>(citations.size());
        int left = 0, right = n - 1;
        int answer = 0;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            int h = n - mid;
            if (citations[mid] >= h) {
                answer = h;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        return answer;
    }
};
