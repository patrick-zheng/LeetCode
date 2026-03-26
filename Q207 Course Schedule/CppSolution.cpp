#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    bool canFinish(int numCourses, vector<vector<int>>& prerequisites) {
        vector<vector<int>> graph(numCourses);
        vector<int> indegree(numCourses, 0);

        for (const auto& p : prerequisites) {
            int a = p[0], b = p[1];
            graph[b].push_back(a);
            ++indegree[a];
        }

        queue<int> q;
        for (int i = 0; i < numCourses; ++i) {
            if (indegree[i] == 0) q.push(i);
        }

        int taken = 0;

        while (!q.empty()) {
            int cur = q.front();
            q.pop();
            ++taken;

            for (int nxt : graph[cur]) {
                if (--indegree[nxt] == 0) {
                    q.push(nxt);
                }
            }
        }

        return taken == numCourses;
    }
};
