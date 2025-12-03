#include <vector>
#include <string>
#include <unordered_map>

using namespace std;


// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (!node) return nullptr;
        return dfs(node);
    }

private:
    std::unordered_map<Node*, Node*> cloned; // original -> clone

    Node* dfs(Node* node) {
        // If this node is already cloned, return the clone
        if (cloned.count(node)) return cloned[node];

        // Create a new node with the same value
        Node* copy = new Node(node->val);
        cloned[node] = copy;

        // Clone all neighbors
        for (Node* nei : node->neighbors) {
            copy->neighbors.push_back(dfs(nei));
        }

        return copy;
    }
};