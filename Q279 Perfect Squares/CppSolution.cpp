#include <queue>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
    int numSquares(int n) {
        vector<int> squares;
        for (int j = 1; j * j <= n; ++j) {
            squares.push_back(j * j);
        }

        queue<int> q;
        q.push(n);
        unordered_set<int> visited = {n};
        int steps = 0;

        while (!q.empty()) {
            ++steps;
            int size = q.size();
            for (int s = 0; s < size; ++s) {
                int value = q.front();
                q.pop();
                for (int sq : squares) {
                    int next = value - sq;
                    if (next == 0) {
                        return steps;
                    }
                    if (next > 0 && !visited.count(next)) {
                        visited.insert(next);
                        q.push(next);
                    }
                }
            }
        }
        return steps;
    }
};
