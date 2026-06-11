#include <sstream>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
    bool wordPattern(string pattern, string s) {
        istringstream stream(s);
        vector<string> words;
        string word;
        while (stream >> word) {
            words.push_back(word);
        }
        if (pattern.size() != words.size()) {
            return false;
        }

        unordered_map<char, string> char_to_word;
        unordered_map<string, char> word_to_char;
        for (int i = 0; i < static_cast<int>(pattern.size()); ++i) {
            char ch = pattern[i];
            const string& w = words[i];
            auto it = char_to_word.find(ch);
            if (it != char_to_word.end()) {
                if (it->second != w) {
                    return false;
                }
            } else if (word_to_char.count(w)) {
                return false;
            } else {
                char_to_word[ch] = w;
                word_to_char[w] = ch;
            }
        }
        return true;
    }
};
