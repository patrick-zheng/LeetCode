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
    ListNode* deleteDuplicates(ListNode* head) {
        ListNode dummy(0, head);
        ListNode* prev = &dummy;
        ListNode* cur = head;

        while (cur) {
            bool dup = false;
            while (cur->next && cur->next->val == cur->val) {
                dup = true;
                ListNode* to_del = cur->next;
                cur->next = to_del->next;
            }
            if (dup) {
                prev->next = cur->next;
                cur = prev->next;
            } else {
                prev = cur;
                cur = cur->next;
            }
        }
        return dummy.next;
    }
};
