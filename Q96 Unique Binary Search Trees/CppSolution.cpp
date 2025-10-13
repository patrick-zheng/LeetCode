#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int numTrees(int n) {
        vector<long long> G(n + 1, 0);
        G[0] = G[1] = 1;
        for (int nodes = 2; nodes <= n; ++nodes) {
            long long total = 0;
            for (int root = 1; root <= nodes; ++root) {
                total += G[root - 1] * G[nodes - root];
            }
            G[nodes] = total;
        }
        return (int)G[n];
    }
};
