#include <vector>
#include <string>

using namespace std;

class WordDictionary {
private:
    struct TrieNode {
        TrieNode* children[26];
        bool isEnd;

        TrieNode() : isEnd(false) {
            for (int i = 0; i < 26; ++i) children[i] = nullptr;
        }
    };

    TrieNode* root;

    bool dfs(TrieNode* node, const string& word, int i) {
        if (!node) return false;
        if (i == (int)word.size()) return node->isEnd;

        if (word[i] == '.') {
            for (int j = 0; j < 26; ++j) {
                if (node->children[j] && dfs(node->children[j], word, i + 1)) {
                    return true;
                }
            }
            return false;
        }

        int idx = word[i] - 'a';
        return dfs(node->children[idx], word, i + 1);
    }

public:
    WordDictionary() {
        root = new TrieNode();
    }

    void addWord(string word) {
        TrieNode* node = root;
        for (char ch : word) {
            int idx = ch - 'a';
            if (!node->children[idx]) {
                node->children[idx] = new TrieNode();
            }
            node = node->children[idx];
        }
        node->isEnd = true;
    }

    bool search(string word) {
        return dfs(root, word, 0);
    }
};
