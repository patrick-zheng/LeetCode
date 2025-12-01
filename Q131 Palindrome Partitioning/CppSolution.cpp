#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<vector<string>> partition(string s) {
        vector<vector<string>> res;
        vector<string> path;
        backtrack(0, s, path, res);
        return res;
    }

private:
    void backtrack(int start, const string &s, vector<string> &path, vector<vector<string>> &res) {
        int n = s.size();
        if (start == n) {
            res.push_back(path);
            return;
        }

        // Try all possible end positions for the next palindrome substring
        for (int end = start; end < n; ++end) {
            if (isPalindrome(s, start, end)) {
                path.push_back(s.substr(start, end - start + 1)); // choose
                backtrack(end + 1, s, path, res);                 // explore
                path.pop_back();                                  // un-choose
            }
        }
    }

    bool isPalindrome(const string &s, int left, int right) {
        while (left < right) {
            if (s[left] != s[right]) return false;
            ++left;
            --right;
        }
        return true;
    }
};
