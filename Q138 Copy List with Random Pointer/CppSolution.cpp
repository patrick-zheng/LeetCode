#include <vector>
#include <string>

using namespace std;

// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (!head) return nullptr;

        // 1) Interleave cloned nodes after each original node
        Node* curr = head;
        while (curr) {
            Node* copy = new Node(curr->val);
            copy->next = curr->next;
            curr->next = copy;
            curr = copy->next;
        }

        // 2) Set random pointers for the copied nodes
        curr = head;
        while (curr) {
            Node* copy = curr->next;
            if (curr->random) {
                copy->random = curr->random->next;  // because random's copy is right after it
            }
            curr = copy->next;
        }

        // 3) Detach the copied list from the original list
        curr = head;
        Node* copyHead = head->next;
        while (curr) {
            Node* copy = curr->next;
            curr->next = copy->next;              // restore original next
            if (copy->next) {
                copy->next = copy->next->next;    // set copy next
            }
            curr = curr->next;
        }

        return copyHead;
    }
};
