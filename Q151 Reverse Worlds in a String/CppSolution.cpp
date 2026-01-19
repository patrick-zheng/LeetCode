#include <string>
#include <sstream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    string reverseWords(string s) {
        istringstream iss(s);
        vector<string> words;
        string word;

        // Step 1: Extract words (automatically ignores extra spaces)
        while (iss >> word) {
            words.push_back(word);
        }

        // Step 2: Reverse the word order
        reverse(words.begin(), words.end());

        // Step 3: Join words with single spaces
        string result;
        for (size_t i = 0; i < words.size(); ++i) {
            if (i > 0) result += " ";
            result += words[i];
        }

        return result;
    }
};
