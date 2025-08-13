#include <functional>

using namespace std;

class Solution {
public:
    int totalNQueens(int n) {
        const int limit = (1 << n) - 1;

        function<int(int,int,int)> dfs = [&](int cols, int d1, int d2) -> int {
            if (cols == limit) return 1;
            int cnt = 0;
            int avail = ~(cols | d1 | d2) & limit;
            while (avail) {
                int p = avail & -avail;      // pick lowest set bit
                avail -= p;
                cnt += dfs(cols | p, ((d1 | p) << 1) & limit, (d2 | p) >> 1);
            }
            return cnt;
        };

        return dfs(0, 0, 0);
    }
};
