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
    ListNode* partition(ListNode* head, int x) {
        ListNode beforeDummy(0), afterDummy(0);
        ListNode *before = &beforeDummy, *after = &afterDummy;

        for (ListNode* curr = head; curr != nullptr; ) {
            ListNode* nxt = curr->next;
            curr->next = nullptr;
            if (curr->val < x) {
                before->next = curr;
                before = before->next;
            } else {
                after->next = curr;
                after = after->next;
            }
            curr = nxt;
        }
        before->next = afterDummy.next;
        return beforeDummy.next;
    }
};
