#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    int ladderLength(string beginWord, string endWord, vector<string>& wordList) {
        if (find(wordList.begin(), wordList.end(), endWord) == wordList.end()) {
            return 0;
        }

        int L = beginWord.size();

        unordered_map<string, vector<string>> patternMap;
        patternMap.reserve(wordList.size() * L);

        for (const string& word : wordList) {
            for (int i = 0; i < L; ++i) {
                string pattern = word.substr(0, i) + "*" + word.substr(i + 1);
                patternMap[pattern].push_back(word);
            }
        }

        queue<pair<string, int>> q;
        q.push({beginWord, 1});

        unordered_set<string> visited;
        visited.insert(beginWord);

        while (!q.empty()) {
            auto [current, depth] = q.front();
            q.pop();

            for (int i = 0; i < L; ++i) {
                string pattern = current.substr(0, i) + "*" + current.substr(i + 1);

                auto it = patternMap.find(pattern);
                if (it == patternMap.end()) continue;

                vector<string>& neighbors = it->second;
                for (const string& neighbor : neighbors) {
                    if (neighbor == endWord) {
                        return depth + 1;
                    }
                    if (!visited.count(neighbor)) {
                        visited.insert(neighbor);
                        q.push({neighbor, depth + 1});
                    }
                }

                neighbors.clear();
            }
        }

        return 0;
    }
};
