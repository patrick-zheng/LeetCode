#include <algorithm>
#include <map>
#include <vector>

using namespace std;

class Solution {
public:
    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        vector<pair<int, int>> events;
        events.reserve(buildings.size() * 2);
        for (const auto& b : buildings) {
            int L = b[0], R = b[1], H = b[2];
            events.push_back({L, -H});
            events.push_back({R, H});
        }
        sort(events.begin(), events.end());

        // Height -> multiplicity; track max with rbegin().
        map<int, int> cnt;
        cnt[0] = 1;

        vector<vector<int>> res;
        size_t i = 0;
        while (i < events.size()) {
            int x = events[i].first;
            while (i < events.size() && events[i].first == x) {
                int h = events[i].second;
                if (h < 0) {
                    int height = -h;
                    ++cnt[height];
                } else {
                    int height = h;
                    if (--cnt[height] == 0) cnt.erase(height);
                }
                ++i;
            }
            int maxH = cnt.rbegin()->first;
            if (res.empty() || res.back()[1] != maxH) {
                res.push_back({x, maxH});
            }
        }
        return res;
    }
};
