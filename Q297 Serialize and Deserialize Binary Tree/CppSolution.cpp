#include <sstream>
#include <string>
#include <vector>

using namespace std;

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* left, TreeNode* right)
        : val(x), left(left), right(right) {}
};

class Codec {
    void preorder(TreeNode* root, string& out) {
        if (!root) {
            out += "N,";
            return;
        }
        out += to_string(root->val) + ",";
        preorder(root->left, out);
        preorder(root->right, out);
    }

    TreeNode* build(const vector<string>& tokens, int& index) {
        if (tokens[index] == "N") {
            ++index;
            return nullptr;
        }
        TreeNode* node = new TreeNode(stoi(tokens[index++]));
        node->left = build(tokens, index);
        node->right = build(tokens, index);
        return node;
    }

public:
    string serialize(TreeNode* root) {
        string out;
        preorder(root, out);
        return out;
    }

    TreeNode* deserialize(string data) {
        vector<string> tokens;
        stringstream ss(data);
        string token;
        while (getline(ss, token, ',')) {
            tokens.push_back(token);
        }
        int index = 0;
        return build(tokens, index);
    }
};
