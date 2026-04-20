#include <algorithm>

class Solution {
public:
    int computeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2, int by2) {
        const long long area_a = static_cast<long long>(ax2 - ax1) * (ay2 - ay1);
        const long long area_b = static_cast<long long>(bx2 - bx1) * (by2 - by1);
        const int ix1 = std::max(ax1, bx1);
        const int iy1 = std::max(ay1, by1);
        const int ix2 = std::min(ax2, bx2);
        const int iy2 = std::min(ay2, by2);
        long long overlap = 0;
        if (ix1 < ix2 && iy1 < iy2) {
            overlap = static_cast<long long>(ix2 - ix1) * (iy2 - iy1);
        }
        return static_cast<int>(area_a + area_b - overlap);
    }
};
