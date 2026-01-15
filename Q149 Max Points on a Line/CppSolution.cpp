#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int maxPoints(vector<vector<int>>& points) {
        int n = (int)points.size();
        if (n <= 2) return n;

        int ans = 2;

        for (int i = 0; i < n; ++i) {
            // Early stop: even if all remaining points align with points[i], can't beat ans
            if (ans >= n - i) break;

            unordered_map<long long, int> cnt; // packed (dy, dx) -> count
            int dup = 0, best = 0;

            long long x1 = points[i][0], y1 = points[i][1];

            for (int j = i + 1; j < n; ++j) {
                long long dx = (long long)points[j][0] - x1;
                long long dy = (long long)points[j][1] - y1;

                if (dx == 0 && dy == 0) {
                    dup++;
                    continue;
                }

                // Normalize slope (dy, dx)
                if (dx == 0) {
                    dy = 1; dx = 0;                 // vertical
                } else if (dy == 0) {
                    dy = 0; dx = 1;                 // horizontal
                } else {
                    long long g = std::gcd(llabs(dy), llabs(dx));
                    dy /= g; dx /= g;

                    // normalize sign: force dx > 0
                    if (dx < 0) { dx = -dx; dy = -dy; }
                }

                // Pack (dy, dx) into one 64-bit key:
                // shift dy into high 32 bits and dx into low 32 bits
                // Both fit because coords are bounded and normalized.
                long long key = (dy << 32) ^ (dx & 0xffffffffLL);

                int c = ++cnt[key];
                best = max(best, c);
            }

            ans = max(ans, best + dup + 1);
        }

        return ans;
    }
};
