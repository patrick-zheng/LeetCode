#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
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

        vector<int> order;
        while (!q.empty()) {
            int u = q.front();
            q.pop();
            order.push_back(u);

            for (int v : graph[u]) {
                if (--indegree[v] == 0) {
                    q.push(v);
                }
            }
        }

        return order.size() == numCourses ? order : vector<int>{};
    }
};
