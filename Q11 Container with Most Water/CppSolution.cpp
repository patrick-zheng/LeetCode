#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int maxArea(vector<int>& height) {
        int left = 0;
        int right = static_cast<int>(height.size()) - 1;
        int maxVolume = 0;

        while (left < right) {
            int hl = height[left];
            int hr = height[right];
            int width = right - left;
            int currentVolume;

            if (hl < hr) {
                currentVolume = hl * width;
                ++left;
            } else {
                currentVolume = hr * width;
                --right;
            }

            if (currentVolume > maxVolume) {
                maxVolume = currentVolume;
            }
        }

        return maxVolume;
    }
};
