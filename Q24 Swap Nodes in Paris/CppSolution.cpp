#include <vector>
#include <string>

using namespace std;

// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* swapPairs(ListNode* head) {
        if (head == nullptr) { return head; }

        ListNode* dummy = new ListNode();
        dummy->next = head;
        ListNode* curr = dummy;

        while (curr->next && curr->next->next) {
            ListNode* first = curr->next;
            ListNode* second = first->next;

            first->next = second->next;
            second->next = first;
            curr->next = second;

            curr = first;
        }

        return dummy->next;
    }
};
