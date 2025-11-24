#include <string>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <queue>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string>& wordList) {
        unordered_set<string> wordSet(wordList.begin(), wordList.end());
        vector<vector<string>> res;

        // If endWord not in dictionary, no solution
        if (!wordSet.count(endWord)) {
            return res;
        }

        // dist[word] = shortest distance from beginWord
        unordered_map<string, int> dist;
        // parents[word] = list of previous words on any shortest path to word
        unordered_map<string, vector<string>> parents;

        queue<string> q;
        q.push(beginWord);
        dist[beginWord] = 0;

        int wordLen = static_cast<int>(beginWord.size());

        // BFS to build distance and parents
        while (!q.empty()) {
            int levelSize = static_cast<int>(q.size());
            unordered_set<string> levelVisited;

            for (int i = 0; i < levelSize; ++i) {
                string cur = q.front();
                q.pop();
                int curDist = dist[cur];

                string next = cur;
                for (int pos = 0; pos < wordLen; ++pos) {
                    char original = next[pos];

                    for (char c = 'a'; c <= 'z'; ++c) {
                        if (c == original) continue;
                        next[pos] = c;

                        if (!wordSet.count(next)) continue;

                        // First time we see this word
                        if (!dist.count(next)) {
                            dist[next] = curDist + 1;
                            parents[next].push_back(cur);
                            q.push(next);
                            levelVisited.insert(next);
                        }
                        // Another shortest path to this word
                        else if (dist[next] == curDist + 1) {
                            parents[next].push_back(cur);
                        }
                    }

                    next[pos] = original;
                }
            }

            // Remove words visited at this BFS level
            for (const auto& w : levelVisited) {
                wordSet.erase(w);
            }
        }

        // If endWord not reached, return empty
        if (!dist.count(endWord)) {
            return res;
        }

        // Backtrack from endWord to beginWord using parents
        vector<string> path;
        path.push_back(endWord);

        function<void(const string&)> dfs = [&](const string& word) {
            if (word == beginWord) {
                vector<string> seq = path;
                reverse(seq.begin(), seq.end());
                res.push_back(seq);
                return;
            }

            auto it = parents.find(word);
            if (it == parents.end()) return;

            for (const string& p : it->second) {
                path.push_back(p);
                dfs(p);
                path.pop_back();
            }
        };

        dfs(endWord);
        return res;
    }
};