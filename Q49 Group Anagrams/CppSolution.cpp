#include <vector>
#include <string>
#include <unordered_map>
using namespace std;

class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> map;

        for (const string& word : strs) {
            int count[26] = {0};
            for (char c : word) {
                count[c - 'a']++;
            }
            // Convert count array to string key
            string key;
            for (int i = 0; i < 26; ++i) {
                key += to_string(count[i]) + '#';  // '#' separator
            }
            map[key].push_back(word);
        }

        vector<vector<string>> result;
        for (auto& [_, group] : map) {
            result.push_back(move(group));
        }
        return result;
    }
};
