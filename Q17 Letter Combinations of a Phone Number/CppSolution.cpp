#include <vector>
#include <string>
#include <unordered_map>
#include <functional>

using namespace std;

class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};

        unordered_map<char, vector<char>> digitToLetters = {
            {'2', {'a', 'b', 'c'}},
            {'3', {'d', 'e', 'f'}},
            {'4', {'g', 'h', 'i'}},
            {'5', {'j', 'k', 'l'}},
            {'6', {'m', 'n', 'o'}},
            {'7', {'p', 'q', 'r', 's'}},
            {'8', {'t', 'u', 'v'}},
            {'9', {'w', 'x', 'y', 'z'}}
        };

        vector<std::string> result;
        string path;

        function<void(int)> backtrack = [&](int index) {
            if (index == digits.size()) {
                result.push_back(path);
                return;
            }

            for (char letter : digitToLetters[digits[index]]) {
                path.push_back(letter);
                backtrack(index + 1);
                path.pop_back();
            }
        };

        backtrack(0);
        return result;
    }
};
